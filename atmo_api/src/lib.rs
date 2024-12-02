//! Bindings to the ATProto API.
//!
//! These bindings are generated automatically based on the [atproto Lexicons].
//!
//! [atproto Lexicons]: https://github.com/bluesky-social/atproto/tree/main/lexicons
// TODO(dp): box large enum variants
#![allow(clippy::large_enum_variant)]

use std::{error::Error, fmt, marker::PhantomData};

mod generated;
#[cfg(test)]
mod tests;

use atmo_core::xrpc::{self, Request};
use bytes::Bytes;
pub use generated::*;
use http_body_util::{BodyExt, Full};
use url::Url;

pub struct XrpcClient {
    inner: reqwest::Client,
}

impl Default for XrpcClient {
    #[inline]
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl XrpcClient {
    /// Creates an `XrpcClient` with default settings.
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

    /// Creates a builder for an XRPC request.
    ///
    /// Every known XRPC request is represented by a type in the [`api`] module.
    ///
    /// # Example
    ///
    /// To call the `com.atproto.server.createSession` RPC:
    ///
    /// ```no_run
    /// # async fn async_main() {
    /// use atmo_api::XrpcClient;
    /// use atmo_api::com::atproto::server::{create_session, CreateSession};
    /// use url::Url;
    ///
    /// let client = XrpcClient::new();
    ///
    /// let url = Url::parse("https://atproto.example.com").unwrap();
    ///
    /// let result = client
    ///     .request(&url, CreateSession)
    ///     .input(&create_session::Input {
    ///         identifier: "username".into(),
    ///         password: "password".into(),
    ///         auth_factor_token: None,
    ///     })
    ///     .unwrap()
    ///     .send()
    ///     .await;
    /// # }
    /// ```
    pub fn request<R>(&self, base_url: &Url, req: R) -> RequestBuilder<R>
    where
        R: Request,
    {
        let _ = req;

        // TODO(dp): don't assert, custom error type
        assert!(base_url.path().ends_with('/'));

        let url = base_url
            .join("xrpc/")
            .and_then(|u| u.join(R::nsid()))
            .unwrap();

        let builder = self.inner.request(R::method(), url);

        RequestBuilder {
            builder,
            query: None,
            marker: PhantomData,
        }
    }
}

/// A response to an XRPC request.
///
/// A value of this type indicates that the server returned a valid [XRPC] response. The result of
/// the RPC can be retrieved via [`Response::result`].
///
/// [XRPC]: https://atproto.com/specs/xrpc
pub struct Response<R>
where
    R: Request,
{
    parts: http::response::Parts,
    result: Result<R::Output, xrpc::Error<R::RpcError>>,
}

impl<R> Response<R>
where
    R: Request,
{
    #[inline]
    pub fn http_status(&self) -> http::StatusCode {
        self.parts.status
    }

    /// Returns the result of the RPC by reference.
    #[inline]
    pub fn result(&self) -> Result<&R::Output, &xrpc::Error<R::RpcError>> {
        self.result.as_ref()
    }
}

/// An error which occurred during an XRPC call.
#[derive(Debug)]
pub enum ResponseError {
    /// An error occurred in the underlying HTTP request.
    Http(reqwest::Error),
    /// The returned HTTP response was not recognized.
    InvalidXrpc(InvalidXrpcError),
}

/// An error produced when an XRPC response body could not be deserialized.
#[derive(Debug)]
pub struct InvalidXrpcError {
    /// The error produced by deserialization.
    pub error: Box<dyn Error>,
    /// The invalid XRPC response.
    pub response: http::Response<Full<Bytes>>,
}

pub struct RequestBuilder<R> {
    builder: reqwest::RequestBuilder,
    query: Option<String>,
    marker: PhantomData<R>,
}

impl<R> RequestBuilder<R>
where
    R: Request,
{
    pub fn params(mut self, params: &R::Params) -> Result<Self, serde_urlencoded_xrpc::ser::Error> {
        self.query = Some(R::serialize_params(params)?);
        Ok(self)
    }

    pub fn input(mut self, input: &R::Input) -> Result<Self, R::InputError> {
        self.builder = self.builder.body(R::serialize_input(input)?);

        if let Some(enc) = R::input_content_type() {
            self.builder = self.builder.header(http::header::CONTENT_TYPE, enc);
        }

        Ok(self)
    }

    /// Sets the value of the `Content-Type` header for the request.
    #[inline]
    pub fn content_type(mut self, content_type: &str) -> Self {
        self.builder = self
            .builder
            .header(http::header::CONTENT_TYPE, content_type);
        self
    }

    /// Applies XRPC admin authorization to this request.
    ///
    /// From the ATProto specification:
    ///
    /// > _Some administrative XRPC endpoints require authentication with admin privileges. The_
    /// > _current scheme for this is to use HTTP Basic authentication with user "admin" and a_
    /// > _fixed token in the password field, instead of HTTP Bearer auth with a JWT. This means_
    /// > _that admin requests do not have a link to the account or identity of the client beyond_
    /// > _"admin"._
    ///
    /// See the [XRPC Authentication] section of the ATProto specification.
    ///
    /// [XRPC Authentication]: https://atproto.com/specs/xrpc#authentication
    #[inline]
    pub fn admin_auth<T>(mut self, token: T) -> Self
    where
        T: fmt::Display,
    {
        self.builder = self.builder.basic_auth("admin", Some(token));
        self
    }

    /// Applies HTTP `Bearer` authorization to this request.
    ///
    /// From the ATProto specification:
    ///
    /// > _There is also a legacy authentication scheme using HTTP Bearer auth with JWT tokens,_
    /// > _including refresh tokens, described here. Initial login uses the_
    /// > _`com.atproto.server.createSession` endpoint, including the password and an account_
    /// > _identifier (eg, handle or registered email address). This returns a `refreshJwt` (as_
    /// > _well as an initial `accessJwt`)._
    ///
    /// See the [XRPC Authentication] section of the ATProto specification.
    ///
    /// [XRPC Authentication]: https://atproto.com/specs/xrpc#authentication
    #[inline]
    pub fn bearer_auth<T>(mut self, token: T) -> Self
    where
        T: fmt::Display,
    {
        self.builder = self.builder.bearer_auth(token);
        self
    }

    /// Sends the XRPC request.
    ///
    /// This consumes the `RequestBuilder`, returning the response if successful. This method
    /// returns an error if the request could not be sent, or if an error occurred while receiving
    /// the response.
    pub async fn send(self) -> Result<Response<R>, ResponseError> {
        let (cl, req) = self.builder.build_split();

        let mut req = req.unwrap();
        req.url_mut().set_query(self.query.as_deref());

        let resp: http::Response<_> = cl.execute(req).await.map_err(ResponseError::Http)?.into();
        let (parts, body) = resp.into_parts();

        let bytes = body
            .collect()
            .await
            .map_err(ResponseError::Http)?
            .to_bytes();

        let status = parts.status;
        if status.is_client_error() || status.is_server_error() {
            let rpc_error = serde_json::from_slice(&bytes).map_err(|error| {
                // Reconstruct the response.
                let response = http::Response::from_parts(parts.clone(), Full::new(bytes));
                ResponseError::InvalidXrpc(InvalidXrpcError {
                    error: Box::new(error),
                    response,
                })
            })?;

            return Ok(Response {
                parts,
                result: Err(rpc_error),
            });
        }

        let output = R::deserialize_output(&bytes).map_err(|error| {
            // Reconstruct the response.
            let response = http::Response::from_parts(parts.clone(), Full::new(bytes));
            ResponseError::InvalidXrpc(InvalidXrpcError {
                error: Box::new(error),
                response,
            })
        })?;

        Ok(Response {
            parts,
            result: Ok(output),
        })
    }
}
