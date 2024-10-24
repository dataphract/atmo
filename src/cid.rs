use std::str::FromStr;

/// Content Identifiers.
use serde::{de::Error as _, Deserialize, Serialize};

const BASE: multibase::Base = multibase::Base::Base32Lower;

/// CID link value, corresponding to the `cid-link` Lexicon type.
pub struct CidLink(cid::Cid);

/// CID string value, corresponding to the `string` Lexicon type with format `cid`.
pub struct CidString(cid::Cid);

impl Serialize for CidString {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0
            .to_string_of_base(BASE)
            .expect("CIDv1 serialization should never fail")
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CidString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let base_code = s
            .chars()
            .next()
            .ok_or_else(|| D::Error::custom("empty CID string"))?;

        let base = multibase::Base::from_code(base_code).map_err(D::Error::custom)?;

        if base != BASE {
            return Err(D::Error::custom("unsupported CID encoding base"));
        }

        let cid = cid::Cid::from_str(s.as_str()).map_err(D::Error::custom)?;

        if cid.version() != cid::Version::V1 {
            return Err(D::Error::custom("unsupported CID version"));
        }

        Ok(CidString(cid))
    }
}

pub enum Codec {
    Raw = 0x55,
    DagCbor = 0x71,
}

#[cfg(test)]
mod tests {
    use cid::multihash::Multihash;

    use super::*;

    #[test]
    fn cid_string_bad_base() {
        let cid = cid::CidGeneric::<32>::new_v1(
            Codec::Raw as u64,
            Multihash::from_bytes(&[0; 32][..]).unwrap(),
        );
    }
}
