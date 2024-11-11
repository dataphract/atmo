use serde::{Deserialize, Serialize};

/// Nullable type.
///
/// # Rationale
///
/// This type exists to make a distinction between missing values (represented by a normal Rust
/// [`Option`]) and null values, as stipulated by the atproto data model:
///
/// > _In the atproto data model there is a semantic difference between explicitly setting \[a\] map_
/// > _field to null and not including the field at all. Both JSON and CBOR have the same_
/// > _distinction._
///
/// A `Nullable` value can be constructed using [`Nullable::null`] or [`Nullable::non_null`] as
/// appropriate. To keep this abstraction as lightweight as possible, the only other implementations
/// are `From<Option<T>>`, `Into<Option<T>>`, `Serialize` and `Deserialize`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Nullable<T>(Option<T>);

impl<T> Nullable<T> {
    #[inline]
    pub fn null() -> Self {
        Nullable(None)
    }

    #[inline]
    pub fn non_null(value: T) -> Self {
        Nullable(Some(value))
    }
}

impl<T> From<Nullable<T>> for Option<T> {
    #[inline]
    fn from(value: Nullable<T>) -> Self {
        value.0
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    #[inline]
    fn from(value: Option<T>) -> Self {
        Nullable(value)
    }
}

impl<T> Serialize for Nullable<T>
where
    Option<T>: Serialize,
{
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(ser)
    }
}

impl<'de, T> Deserialize<'de> for Nullable<T>
where
    Option<T>: Deserialize<'de>,
{
    #[inline]
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(des).map(Nullable)
    }
}
