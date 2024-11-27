//! ATProto-specific URI support.

use std::{num::NonZeroU16, ops::RangeInclusive, str::FromStr};

use serde::Serialize;

use crate::{
    error::ParseError, handle::Handle, impl_deserialize_via_from_str, nsid::Nsid,
    parse::subslice_range, rkey::RecordKey,
};

const LEN_RANGE: RangeInclusive<usize> = 1..=(8 * 1024);

/// An AT-URI.
///
/// This type specifically represents the "restricted" AT-URI used by Lexicon, matching the pattern:
///
/// ```text
/// "at://" AUTHORITY [ "/" COLLECTION [ "/" RKEY ] ]
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtUri {
    text: String,
    segments: Segments,
}

impl AtUri {
    const SCHEME: &[u8] = b"at://";

    /// Returns the authority segment of the AT-URI.
    pub fn authority(&self) -> &str {
        let start = Self::SCHEME.len();

        let Some(coll_start) = self.segments.collection_start else {
            return &self.text[start..];
        };

        let end = usize::from(coll_start.get() - 1);
        &self.text[start..end]
    }

    /// Returns the collection segment of the AT-URI, if any.
    pub fn collection(&self) -> Option<&str> {
        let coll_start: usize = self.segments.collection_start?.get().into();

        let coll_end = match self.segments.rkey_start {
            Some(idx) => usize::from(idx.get()) - 1,
            None => self.text.len(),
        };

        Some(&self.text[coll_start..coll_end])
    }

    /// Returns the record key segment of the AT-URI, if any.
    pub fn rkey(&self) -> Option<&str> {
        let rkey_start: usize = self.segments.rkey_start?.get().into();

        Some(&self.text[rkey_start..])
    }
}

impl FromStr for AtUri {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments = Segments::from_str(s)?;

        Ok(AtUri {
            text: s.into(),
            segments,
        })
    }
}

impl_deserialize_via_from_str!(AtUri);

impl Serialize for AtUri {
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.text.serialize(ser)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Segments {
    collection_start: Option<NonZeroU16>,
    rkey_start: Option<NonZeroU16>,
}

impl FromStr for Segments {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let all_bytes = s.as_bytes();

        if !LEN_RANGE.contains(&all_bytes.len()) {
            return Err(ParseError::at_uri());
        }

        // Strip scheme.
        let bytes = all_bytes
            .strip_prefix(AtUri::SCHEME)
            .ok_or_else(ParseError::at_uri)?;

        let mut it = bytes.split(|&c| c == b'/');

        let authority = it.next().ok_or_else(ParseError::at_uri)?;
        if authority.starts_with(b"did:") {
            crate::did::validate_did(authority)?;
        } else {
            Handle::new(std::str::from_utf8(authority).unwrap()).ok_or_else(ParseError::at_uri)?;
        }

        let Some(collection) = it.next() else {
            return Ok(Segments {
                collection_start: None,
                rkey_start: None,
            });
        };

        Nsid::try_from(collection)?;

        let cr = subslice_range(all_bytes, collection).unwrap();
        let collection_start = NonZeroU16::new(cr.start as u16);

        let Some(rkey) = it.next() else {
            return Ok(Segments {
                collection_start,
                rkey_start: None,
            });
        };

        let rr = subslice_range(all_bytes, rkey).unwrap();
        let rkey_start = NonZeroU16::new(rr.start as u16);

        RecordKey::try_from(rkey)?;

        Ok(Segments {
            collection_start,
            rkey_start,
        })
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

    #[test]
    fn segment_getters() {
        let uri: AtUri = "at://example.com".parse().unwrap();
        assert_eq!(uri.collection(), None);
        assert_eq!(uri.rkey(), None);

        let uri: AtUri = "at://example.com/com.example.foo".parse().unwrap();
        assert_eq!(uri.collection(), Some("com.example.foo"));
        assert_eq!(uri.rkey(), None);

        let uri: AtUri = "at://example.com/com.example.foo/9001".parse().unwrap();
        assert_eq!(uri.collection(), Some("com.example.foo"));
        assert_eq!(uri.rkey(), Some("9001"));
    }
}
