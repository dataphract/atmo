use std::{ops::RangeInclusive, str::FromStr};

use serde::{de::Error as _, Deserialize, Serialize};

use crate::error::ParseError;

pub use crate::{
    at_uri::AtUri,
    blob::Blob,
    cid::{CidLink, CidString},
    datetime::DateTimeString,
    did::Did,
    handle::Handle,
    nsid::Nsid,
    nullable::Nullable,
    rkey::RecordKey,
    tid::Tid,
    unknown::Unknown,
};

pub mod at_uri;
mod blob;
pub mod bytes;
mod cid;
pub mod datetime;
pub mod did;
pub mod error;
pub mod handle;
pub mod nsid;
mod nullable;
mod parse;
pub mod rkey;
pub mod tid;
mod unknown;
pub mod xrpc;

pub(crate) const SEGMENT_LEN_RANGE: RangeInclusive<usize> = 1..=63;

pub enum AtIdentifier {
    Did(Did),
    Handle(Handle),
}

impl FromStr for AtIdentifier {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Did::from_str(s)
            .map(AtIdentifier::Did)
            .or_else(|_| Handle::from_str(s).map(AtIdentifier::Handle))
            .map_err(|_| ParseError::at_identifier())
    }
}

impl<'de> Deserialize<'de> for AtIdentifier {
    #[inline]
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(des)?;
        AtIdentifier::from_str(s).map_err(D::Error::custom)
    }
}

impl Serialize for AtIdentifier {
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AtIdentifier::Did(did) => did.serialize(ser),
            AtIdentifier::Handle(handle) => handle.serialize(ser),
        }
    }
}

// Taken from stdlib until slice::split_once is stable.
#[inline]
pub(crate) fn split_once<F>(slice: &[u8], pred: F) -> Option<(&[u8], &[u8])>
where
    F: FnMut(&u8) -> bool,
{
    let index = slice.iter().position(pred)?;
    Some((&slice[..index], &slice[index + 1..]))
}

#[inline]
pub(crate) fn is_valid_tld(s: &[u8]) -> bool {
    is_valid_domain_segment(s) && s[0].is_ascii_alphabetic()
}

#[inline]
pub(crate) fn is_valid_domain_segment(s: &[u8]) -> bool {
    SEGMENT_LEN_RANGE.contains(&s.len())
        && s[0] != b'-'
        && s[s.len() - 1] != b'-'
        && s.iter().all(is_segment_char)
}

fn is_segment_char(b: &u8) -> bool {
    b.is_ascii_alphanumeric() || *b == b'-'
}

#[inline]
pub(crate) fn is_valid_nsid_name(s: &[u8]) -> bool {
    SEGMENT_LEN_RANGE.contains(&s.len())
        && s[0] != b'-'
        && s[s.len() - 1] != b'-'
        && s.iter().all(|b| b.is_ascii_alphabetic())
}

#[cfg(test)]
pub(crate) mod test {
    use std::str::FromStr;

    pub fn test_valid<'a, T>(cases: impl IntoIterator<Item = &'a str>)
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let typename = std::any::type_name::<T>();

        for case in cases {
            if let Err(e) = T::from_str(case) {
                panic!("valid {typename} rejected: {e} (input: {case:?})");
            }
        }
    }

    pub fn test_invalid<'a, T>(cases: impl IntoIterator<Item = &'a str>)
    where
        T: FromStr,
        T::Err: std::fmt::Display,
    {
        let typename = std::any::type_name::<T>();

        for case in cases {
            if T::from_str(case).is_ok() {
                panic!("invalid {typename} accepted: {case:?}");
            }
        }
    }
}