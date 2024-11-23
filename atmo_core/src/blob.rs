use std::num::NonZeroU64;

use serde::{
    de::{Error as _, Unexpected},
    Deserialize, Serialize,
};

use crate::cid::CidLink;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Blob {
    pub ref_: CidLink,
    pub mime_type: String,
    pub size: NonZeroU64,
}

impl Serialize for Blob {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct SerializeBlob<'a> {
            #[serde(rename = "$type")]
            ty: &'static str,
            #[serde(rename = "ref")]
            ref_: &'a CidLink,
            #[serde(rename = "mimeType")]
            mime_type: &'a String,
            size: &'a NonZeroU64,
        }

        SerializeBlob {
            ty: "blob",
            ref_: &self.ref_,
            mime_type: &self.mime_type,
            size: &self.size,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Blob {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct DeserializeBlob {
            #[serde(rename = "$type")]
            ty: String,
            #[serde(rename = "ref")]
            ref_: CidLink,
            #[serde(rename = "mimeType")]
            mime_type: String,
            size: NonZeroU64,
        }

        let db = DeserializeBlob::deserialize(deserializer)?;

        if db.ty != "blob" {
            return Err(D::Error::invalid_value(
                Unexpected::Str(db.ty.as_str()),
                &"blob",
            ));
        }

        Ok(Blob {
            ref_: db.ref_,
            mime_type: db.mime_type,
            size: db.size,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest::proptest! {
        #[test]
        fn proptest_blob_roundtrip(blob: Blob) {
            let serialized = serde_json::to_string(&blob).unwrap();
            let deserialized = serde_json::from_str(&serialized).unwrap();

            assert_eq!(blob, deserialized);
        }
    }
}
