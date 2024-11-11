use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Nsid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Unknown {
    #[serde(default, rename = "$type")]
    ty: Option<Nsid>,
    #[serde(flatten)]
    inner: serde_json::Map<String, serde_json::Value>,
}

impl Unknown {
    pub fn ty(&self) -> Option<&Nsid> {
        self.ty.as_ref()
    }

    pub fn downcast<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        T::deserialize(&self.inner)
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
