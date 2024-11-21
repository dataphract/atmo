//! A crate for interacting with the [AT Protocol].
//!
//! [AT Protocol]: https://atproto.com/

use std::fmt;

use atmo_core::xrpc::{self, Request};
use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use serde::Serialize;
use url::Url;

pub mod api {
    pub use atmo_api::*;
}

pub mod core {
    pub use atmo_core::*;
}

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
    /// use atmo::XrpcClient;
    /// use atmo::api::com::atproto::server::{create_session, CreateSession};
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
    ///     .send()
    ///     .await;
    /// # }
    /// ```
    pub fn request<R>(&self, base_url: &Url, req: R) -> RequestBuilder<R>
    where
        R: Request,
    {
        // TODO(dp): don't assert, custom error type
        assert!(base_url.path().ends_with('/'));

        let url = base_url
            .join("xrpc/")
            .and_then(|u| u.join(R::nsid()))
            .unwrap();

        let builder = self.inner.request(R::method(), url);

        RequestBuilder { builder, _req: req }
    }
}

pub struct RequestBuilder<R> {
    builder: reqwest::RequestBuilder,
    _req: R,
}

impl<R> RequestBuilder<R>
where
    R: Request,
    R::Params: Serialize,
{
    #[inline]
    pub fn params(mut self, params: &R::Params) -> Self {
        self.builder = self.builder.query(params);
        self
    }
}

impl<R> RequestBuilder<R>
where
    R: Request,
    R::Input: Serialize,
{
    #[inline]
    pub fn input(mut self, input: &R::Input) -> Self {
        self.builder = self.builder.json(input);
        self
    }
}

impl<R> RequestBuilder<R>
where
    R: Request,
{
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
}

impl<R> RequestBuilder<R>
where
    R: Request,
{
    /// Sends the XRPC request.
    ///
    /// This consumes the `RequestBuilder`, returning the response if successful. This method
    /// returns an error if the request could not be sent, or if an error occurred while receiving
    /// the response.
    pub async fn send(self) -> Result<Response<R>, ResponseError> {
        let resp: http::Response<reqwest::Body> = self
            .builder
            .send()
            .await
            .map_err(ResponseError::Http)?
            .into();

        let (parts, body) = resp.into_parts();

        let bytes = body
            .collect()
            .await
            .map_err(ResponseError::Http)?
            .to_bytes();

        let status = parts.status;
        if status.is_client_error() || status.is_server_error() {
            let rpc_error = serde_json::from_slice(&bytes).map_err(|_| {
                // Reconstruct the response.
                let resp = http::Response::from_parts(parts.clone(), Full::new(bytes));
                ResponseError::InvalidXrpc(resp)
            })?;

            return Ok(Response {
                parts,
                result: Err(rpc_error),
            });
        }

        let output = serde_json::from_slice(&bytes).map_err(|_| {
            // Reconstruct the response.
            let resp = http::Response::from_parts(parts.clone(), Full::new(bytes));
            ResponseError::InvalidXrpc(resp)
        })?;

        Ok(Response {
            parts,
            result: Ok(output),
        })
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
    result: Result<R::Output, xrpc::Error<R::Error>>,
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
    pub fn result(&self) -> Result<&R::Output, &xrpc::Error<R::Error>> {
        self.result.as_ref()
    }
}

/// An error which occurred during an XRPC call.
#[derive(Debug)]
pub enum ResponseError {
    /// An error occurred in the underlying HTTP request.
    Http(reqwest::Error),
    /// The returned HTTP response was not recognized.
    InvalidXrpc(http::Response<Full<Bytes>>),
}
