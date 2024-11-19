use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Request {
    type Params;
    type Input;
    type Output: DeserializeOwned;
    type Error: DeserializeOwned;

    fn method() -> http::Method;
    fn nsid() -> &'static str;
    fn output_encoding() -> &'static str;
}

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
