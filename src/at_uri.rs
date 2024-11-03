use std::{num::NonZeroU16, ops::RangeInclusive, str::FromStr};

use serde::{de::Error as _, Deserialize, Serialize};

use crate::{
    did::Did, error::ParseError, handle::Handle, nsid::Nsid, parse::subslice_range, rkey::RecordKey,
};

const LEN_RANGE: RangeInclusive<usize> = 1..=(8 * 1024);

/// An AT-URI.
///
/// This type specifically represents the "restricted" AT-URI used by Lexicon, matching the pattern:
///
/// ```text
/// "at://" AUTHORITY [ "/" COLLECTION [ "/" RKEY ] ]
/// ```
pub struct AtUri {
    // The full text of the AT-URI.
    text: String,
    // Start index of the path segment.
    collection_start: Option<NonZeroU16>,
    // Start index of the rkey segment.
    rkey_start: Option<NonZeroU16>,
}

impl AtUri {
    const SCHEME: &[u8] = b"at://";

    pub fn new(uri: String) -> Option<AtUri> {
        if !LEN_RANGE.contains(&uri.len()) {
            return None;
        }

        // Strip scheme.
        let all_bytes = uri.as_bytes();
        let bytes = all_bytes.strip_prefix(Self::SCHEME)?;

        let mut it = bytes.split(|&c| c == b'/');

        let authority = it.next()?;
        if authority.starts_with(b"did:") {
            Did::new(authority)?;
        } else {
            Handle::new(std::str::from_utf8(authority).unwrap())?;
        }

        let Some(collection) = it.next() else {
            return Some(AtUri {
                collection_start: None,
                rkey_start: None,
                text: uri,
            });
        };

        Nsid::try_from(collection).ok()?;

        let cr = subslice_range(all_bytes, collection).unwrap();
        let collection_start = NonZeroU16::new(cr.start as u16);

        let Some(rkey) = it.next() else {
            return Some(AtUri {
                collection_start,
                rkey_start: None,
                text: uri,
            });
        };

        let rr = subslice_range(all_bytes, rkey).unwrap();
        let rkey_start = NonZeroU16::new(rr.start as u16);

        RecordKey::try_from(rkey).ok()?;

        Some(AtUri {
            collection_start,
            rkey_start,
            text: uri,
        })
    }

    pub fn authority(&self) -> &str {
        let start = Self::SCHEME.len();

        let Some(coll_start) = self.collection_start else {
            return &self.text[start..];
        };

        let end = usize::from(coll_start.get() - 1);
        &self.text[start..end]
    }

    pub fn collection(&self) -> Option<&str> {
        let coll_start: usize = self.collection_start?.get().into();

        let coll_end = match self.rkey_start {
            Some(idx) => usize::from(idx.get()) - 1,
            None => self.text.len(),
        };

        Some(&self.text[coll_start..coll_end])
    }
}

impl FromStr for AtUri {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AtUri::new(s.to_string()).ok_or_else(ParseError::at_uri)
    }
}

impl<'de> Deserialize<'de> for AtUri {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(des)?;
        AtUri::from_str(s).map_err(D::Error::custom)
    }
}

impl Serialize for AtUri {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.text.serialize(ser)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_examples() {
        crate::test::test_valid::<AtUri>(["at://foo.com/com.example.foo/123"]);
    }

    #[test]
    fn invalid_restricted_examples() {
        crate::test::test_invalid::<AtUri>([
            "at://foo.com/example/123", // invalid NSID
            "at://computer",            // not a valid DID or handle
            "at://example.com:3000",    // not a valid DID or handle
        ]);
    }

    #[test]
    fn invalid_examples() {
        crate::test::test_invalid::<AtUri>([
            "at://foo.com/",          // trailing slash
            "at://user:pass@foo.com", // userinfo not currently supported
        ])
    }
}
