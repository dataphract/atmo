//! Content Identifiers.

use std::str::FromStr;

use cid::multibase;
use serde::{de::Error as _, ser::SerializeStruct, Deserialize, Serialize};

const BASE: multibase::Base = multibase::Base::Base32Lower;

/// CID link value, corresponding to the `cid-link` Lexicon type.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CidLink(cid::Cid);

impl Serialize for CidLink {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if ser.is_human_readable() {
            let s = self
                .0
                .to_string_of_base(BASE)
                .expect("CIDv1 serialization should never fail");

            let mut struct_ser = ser.serialize_struct("CidLink", 1)?;
            struct_ser.serialize_field("$link", s.as_str())?;
            struct_ser.end()
        } else {
            // serde_ipld_dagcbor recognizes this by magic (CID_SERDE_PRIVATE_IDENTIFIER).
            self.0.serialize(ser)
        }
    }
}

impl<'de> Deserialize<'de> for CidLink {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if des.is_human_readable() {
            #[derive(Deserialize)]
            struct Link {
                #[serde(rename = "$link")]
                link: CidString,
            }

            Link::deserialize(des).map(|l| CidLink(l.link.0))
        } else {
            // serde_ipld_dagcbor recognizes this by magic (CID_SERDE_PRIVATE_IDENTIFIER).
            cid::Cid::deserialize(des).map(CidLink)
        }
    }
}

/// CID string value, corresponding to the `string` Lexicon type with format `cid`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use std::iter;

    use cid::multihash::Multihash;

    use super::*;

    // TODO(dp): expose
    enum Codec {
        Raw = 0x55,
        // DagCbor = 0x71,
    }

    fn gen_cid_string(codec: Codec, base: multibase::Base) -> String {
        let mut bytes = Vec::new();
        bytes.push(0x12); // sha2-256
        bytes.push(0x20); // 256-bit digest
        bytes.extend(iter::repeat(0).take(0x20));

        let multihash = Multihash::from_bytes(&bytes).unwrap();

        cid::CidGeneric::<32>::new_v1(codec as u64, multihash)
            .to_string_of_base(base)
            .unwrap()
    }

    #[test]
    fn cid_string_bad_base() {
        let s = gen_cid_string(Codec::Raw, multibase::Base::Base58Btc);

        serde_json::from_value::<CidString>(serde_json::Value::String(s))
            .expect_err("should error with unsupported base");
    }
}
