/// Module for serialization and deserialization of `Bytes` adhering to ATProto.
pub mod serde {
    use std::borrow::Cow;

    use bytes::{Bytes, BytesMut};
    use data_encoding::BASE64;
    use serde::{
        de::Error, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
    };

    #[inline]
    pub fn serialize<S>(bytes: &Bytes, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if ser.is_human_readable() {
            let mut s = ser.serialize_struct("Bytes", 1)?;
            s.serialize_field("$bytes", &BASE64.encode(bytes))?;
            s.end()
        } else {
            bytes.serialize(ser)
        }
    }

    #[inline]
    pub fn deserialize<'de, D>(des: D) -> Result<Bytes, D::Error>
    where
        D: Deserializer<'de>,
    {
        if des.is_human_readable() {
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
            let len = BASE64
                .decode_mut(wrap.bytes.as_bytes(), bytes.as_mut())
                .map_err(|e| D::Error::custom(e.error))?;
            bytes.truncate(len);

            Ok(bytes.freeze())
        } else {
            Bytes::deserialize(des)
        }
    }

    pub mod option {
        use serde::de;

        use super::*;

        #[inline]
        pub fn serialize<S>(opt: &Option<Bytes>, ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match opt {
                Some(b) => super::serialize(b, ser),
                None => ser.serialize_none(),
            }
        }

        #[inline]
        pub fn deserialize<'de, D>(des: D) -> Result<Option<Bytes>, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct Visitor;

            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Option<Bytes>;

                #[inline]
                fn visit_some<D>(self, des: D) -> Result<Self::Value, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    super::deserialize(des).map(Some)
                }

                #[inline]
                fn visit_none<E>(self) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(None)
                }

                #[inline]
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("an `Option<Bytes>`")
                }
            }

            des.deserialize_option(Visitor)
        }
    }
}

#[cfg(test)]
mod tests {
    use data_encoding::BASE64;
    use serde_json::json;

    use super::*;

    #[test]
    fn empty() {
        let input = json!({
            "$bytes": "",
        });

        let bytes = serde::deserialize(input).unwrap();
        assert!(bytes.is_empty());
    }

    #[test]
    fn json_roundtrip() {
        let input = "Hello, world!";
        let base64 = BASE64.encode(input.as_bytes());

        let obj = json!({
            "$bytes": base64,
        });

        let bytes = serde::deserialize(obj).unwrap();
        assert_eq!(input.as_bytes(), &bytes);
    }
}
