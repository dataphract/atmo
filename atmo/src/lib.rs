use std::marker::PhantomData;

use atmo_core::xrpc::Request;
use http::header;
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

pub use atmo_api as api;
pub use atmo_core as core;

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

        let text = resp.text().await.unwrap();
        println!("body text: {text}");

        let out: R::Output = serde_json::from_str(&text).unwrap();

        Ok(out)
    }
}
