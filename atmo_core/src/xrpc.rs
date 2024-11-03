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
