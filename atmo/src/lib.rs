use std::{fmt, marker::PhantomData};

use atmo_core::xrpc::{self, Request};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

pub use atmo_api as api;
pub use atmo_core as core;

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

        RequestBuilder { builder, req }
    }
}

pub struct RequestBuilder<R> {
    builder: reqwest::RequestBuilder,
    req: R,
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
    /// > _`com.atproto.server.createSession endpoint`, including the password and an account_
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
    R::Output: DeserializeOwned,
{
    pub async fn send(self) -> Result<R::Output, reqwest::Error> {
        let resp = self.builder.send().await?;

        if resp.status().is_client_error() || resp.status().is_server_error() {
            let error: xrpc::Error = resp.json().await?;
            panic!("request error: {error:#?}");
        }

        let bytes = resp.bytes().await?;

        let s = String::from_utf8(bytes.into()).unwrap();

        let out: R::Output = serde_json::from_str(&s).unwrap();

        Ok(out)
    }
}
