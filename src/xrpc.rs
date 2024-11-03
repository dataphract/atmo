use std::marker::PhantomData;

use http::header;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{AtIdentifier, Did, Handle};

pub trait Request {
    type Params;
    type Input;
    type Output;

    fn method() -> http::Method;
    fn nsid() -> &'static str;
    fn output_encoding() -> &'static str;
}

pub enum NoParams {}
pub enum NoInput {}
pub enum NoOutput {}

pub struct RequestBuilder<R> {
    builder: reqwest::RequestBuilder,
    marker: PhantomData<R>,
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
    R::Output: DeserializeOwned,
{
    pub async fn send(self) -> Result<R::Output, reqwest::Error> {
        let resp = self.builder.send().await?;

        if let Some(c) = resp.headers().get(header::CONTENT_TYPE) {
            println!("content-type: {}", c.to_str().unwrap());
        }

        println!("status: {}", resp.status().as_u16());

        let out: R::Output = resp.json().await?;

        Ok(out)
    }
}

pub struct XrpcClient {
    inner: reqwest::Client,
}

impl XrpcClient {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::new(),
        }
    }

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

        println!("url: {url}");

        let builder = self.inner.request(R::method(), url);

        RequestBuilder {
            builder,
            marker: PhantomData,
        }
    }
}
