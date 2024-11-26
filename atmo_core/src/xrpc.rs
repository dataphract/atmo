use std::error::Error as StdError;

use bytes::Bytes;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A trait for types which represent an XRPC method.
pub trait Request {
    type Params;

    type Input;
    type InputError: StdError + 'static;

    type Output;
    type OutputError: StdError + 'static;

    /// The type of the response error code returned after an errored request.
    type RpcError: DeserializeOwned;

    /// The HTTP request method used when invoking the XRPC method.
    ///
    /// This is `GET` for queries and `POST` for procedures.
    fn method() -> http::Method;

    /// The unique NSID of the XRPC method.
    fn nsid() -> &'static str;

    /// Serializes this RPC's query parameters to a query string.
    fn serialize_params(params: &Self::Params)
        -> Result<String, serde_urlencoded_xrpc::ser::Error>;

    /// Deserializes this RPC's query parameters from a query string.
    fn deserialize_params(query: &str) -> Result<Self::Params, serde_urlencoded_xrpc::de::Error>;

    /// Returns the media (MIME) type of the request body.
    fn input_content_type() -> Option<&'static str>;

    /// Serializes this RPC's input to a byte buffer.
    fn serialize_input(input: &Self::Input) -> Result<Bytes, Self::InputError>;

    /// Deserializes this RPC's input from a byte buffer.
    fn deserialize_input(bytes: &Bytes) -> Result<Self::Input, Self::InputError>;

    /// Serializes this RPC's output to a byte buffer.
    fn serialize_output(output: &Self::Output) -> Result<Bytes, Self::OutputError>;

    /// Deserializes this RPC's output from a byte buffer.
    fn deserialize_output(bytes: &Bytes) -> Result<Self::Output, Self::OutputError>;
}

/// A generic XRPC error.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Error<E> {
    pub error: E,
    #[serde(default, skip_serializing_if = "std::option::Option::is_none")]
    pub message: Option<String>,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn error_roundtrip() {
        // With message
        let i1 = Error {
            error: String::from("Error"),
            message: Some("Message!".into()),
        };

        let s1 = serde_json::to_value(&i1).unwrap();
        assert_eq!(s1, json!({ "error": "Error", "message": "Message!" }));

        let d1 = serde_json::from_value(s1).unwrap();
        assert_eq!(i1, d1);

        // Without message
        let i2 = Error {
            error: String::from("Error"),
            message: None,
        };

        let s2 = serde_json::to_value(&i2).unwrap();
        assert_eq!(s2, json!({ "error": "Error" }));

        let d2 = serde_json::from_value(s2).unwrap();
        assert_eq!(i2, d2);
    }
}
