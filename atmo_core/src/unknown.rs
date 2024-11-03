use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Nsid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Unknown {
    #[serde(default, rename = "$type")]
    ty: Option<Nsid>,
    #[serde(flatten)]
    inner: serde_json::Value,
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
