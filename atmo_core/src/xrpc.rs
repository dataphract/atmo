use serde::{Deserialize, Serialize};

pub trait Request {
    type Params;
    type Input;
    type Output;
    type Error;

    fn method() -> http::Method;
    fn nsid() -> &'static str;
    fn output_encoding() -> &'static str;
}

pub enum NoParams {}
pub enum NoInput {}
pub enum NoOutput {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    error: String,
    #[serde(default, skip_serializing_if = "std::option::Option::is_none")]
    message: Option<String>,
}
