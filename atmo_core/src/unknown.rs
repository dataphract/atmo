use serde::{de::IntoDeserializer, Deserialize, Serialize};

use crate::Nsid;

/// A dynamically typed ATProto object.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unknown {
    #[serde(default, rename = "$type")]
    ty: Option<Nsid>,
    #[serde(flatten)]
    inner: serde_json::Map<String, serde_json::Value>,
}

impl Unknown {
    /// Returns the type tag of this value, if it has one.
    pub fn ty(&self) -> Option<&Nsid> {
        self.ty.as_ref()
    }
}

impl<'de> IntoDeserializer<'de, serde_json::Error> for &'de Unknown {
    type Deserializer = &'de serde_json::Map<String, serde_json::Value>;

    fn into_deserializer(self) -> Self::Deserializer {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn deserialize_empty() {
        let obj = json!({});

        let unk = Unknown::deserialize(obj).unwrap();

        assert!(unk.ty.is_none());
        assert!(unk.inner.is_empty());
    }
}
