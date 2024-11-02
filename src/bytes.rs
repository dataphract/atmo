use std::borrow::Cow;

use bytes::{Bytes, BytesMut};
use data_encoding::BASE64;
use serde::{de::Error, ser::SerializeStruct, Deserialize, Deserializer, Serializer};

#[inline]
pub fn serialize<S>(bytes: &Bytes, ser: S) -> Result<(), S::Error>
where
    S: Serializer,
{
    let mut s = ser.serialize_struct("Bytes", 1)?;
    s.serialize_field("$bytes", &BASE64.encode(bytes))?;
    s.end()?;

    Ok(())
}

#[inline]
pub fn deserialize<'de, D>(des: D) -> Result<Bytes, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper<'a> {
        #[serde(rename = "$bytes")]
        bytes: Cow<'a, str>,
    }

    let wrap = Wrapper::deserialize(des)?;

    let len = BASE64
        .decode_len(wrap.bytes.len())
        .map_err(D::Error::custom)?;

    let mut bytes = BytesMut::with_capacity(len);
    bytes.resize(len, 0);
    BASE64
        .decode_mut(wrap.bytes.as_bytes(), bytes.as_mut())
        .map_err(|e| D::Error::custom(e.error))?;

    Ok(bytes.freeze())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn empty() {
        let input = json!({
            "$bytes": "",
        });

        let bytes = deserialize(input).unwrap();
        assert!(bytes.is_empty());
    }
}
