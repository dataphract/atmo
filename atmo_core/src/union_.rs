//! Support for deserializing ATProto unions.

use std::{
    borrow::Cow,
    collections::{btree_map::Entry, BTreeMap},
    fmt,
};

use ipld_core::ipld::Ipld;
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Serialize,
};

#[derive(Serialize)]
pub struct UnionSerialize<'a> {
    #[serde(rename = "$type")]
    pub ty: &'static str,
    #[serde(flatten)]
    pub map: &'a dyn erased_serde::Serialize,
}

pub struct UnionRef<'a, T> {
    pub ty: Cow<'a, str>,
    pub map: BTreeMap<Cow<'a, str>, T>,
}

impl<'de, T> Deserialize<'de> for UnionRef<'de, T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        des.deserialize_map(UnionVisitor::<T>::default())
    }
}

pub struct UnionVisitor<'a, T> {
    ty: Option<Cow<'a, str>>,
    pairs: BTreeMap<Cow<'a, str>, T>,
}

impl<'a, T> Default for UnionVisitor<'a, T> {
    #[inline]
    fn default() -> Self {
        Self {
            ty: Default::default(),
            pairs: Default::default(),
        }
    }
}

impl<'a, T> UnionVisitor<'a, T> {
    fn set_type<E>(&mut self, v: Cow<'a, str>) -> Result<(), E>
    where
        E: serde::de::Error,
    {
        if self.ty.is_some() {
            return Err(E::duplicate_field("$type"));
        }

        self.ty = Some(v);

        Ok(())
    }

    fn into_union_ref<E>(self) -> Result<UnionRef<'a, T>, E>
    where
        E: serde::de::Error,
    {
        let ty = self.ty.ok_or_else(|| E::missing_field("$type"))?;

        Ok(UnionRef {
            ty,
            map: self.pairs,
        })
    }
}

impl<'de, T> Visitor<'de> for UnionVisitor<'de, T>
where
    T: Deserialize<'de>,
{
    type Value = UnionRef<'de, T>;

    fn visit_map<A>(mut self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        use serde::de::Error;

        while let Some(key) = map.next_key::<Cow<str>>()? {
            if key == "$type" {
                // Type tag should always be a string.
                let value: Cow<str> = map.next_value()?;

                // Record the type tag, erroring if it's already set.
                self.set_type::<A::Error>(value)?;
            } else {
                // Add all other values to the map.
                match self.pairs.entry(key.clone()) {
                    Entry::Vacant(v) => v.insert(map.next_value()?),
                    Entry::Occupied(_) => {
                        return Err(A::Error::custom(format!("duplicate key in map: `{key}`")));
                    }
                };
            }
        }

        // Convert to a UnionRef, erroring if no type tag was recorded.
        self.into_union_ref()
    }

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a map containing a `$type` field")
    }
}

pub struct IpldIntoDeserializer(pub Ipld);

impl<'de> serde::de::IntoDeserializer<'de, ipld_core::serde::SerdeError> for IpldIntoDeserializer {
    type Deserializer = Ipld;

    fn into_deserializer(self) -> Self::Deserializer {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, value::RawValue};

    use super::*;

    #[test]
    fn union_ref_empty_map_json() {
        let map = json!({});

        assert!(UnionRef::<&RawValue>::deserialize(map).is_err());
    }

    #[test]
    fn union_ref_multiple_type_tags_json() {
        // The json!() macro doesn't keep duplicate fields, so we have to use a string instead.
        let s = r#"{ "$type": "foo", "$type": "bar" }"#;

        assert!(serde_json::from_str::<UnionRef<&RawValue>>(s).is_err());
    }

    #[test]
    fn union_ref_missing_type_tag_json() {
        let map = json!({
            "foo": 1,
            "bar": [42],
        });

        assert!(UnionRef::<&RawValue>::deserialize(map).is_err());
    }

    #[test]
    fn union_ref_only_type_tag_json() {
        let ty = "com.example.foo.barBaz";

        let map = json!({
            "$type": ty,
        });

        let u = UnionRef::<&RawValue>::deserialize(map).unwrap();

        assert_eq!(u.ty, ty);
        assert!(u.map.is_empty());
    }

    #[test]
    fn union_serialize() {
        #[derive(Serialize)]
        struct Example {
            foo: String,
            bar: u64,
        }

        let serialized = serde_json::to_value(UnionSerialize {
            ty: "com.example.foo.bar",
            map: &Example {
                foo: "Hello".into(),
                bar: 42,
            },
        })
        .unwrap();

        let expected = json!({
            "$type": "com.example.foo.bar",
            "foo": "Hello",
            "bar": 42,
        });

        assert_eq!(serialized, expected);
    }
}
