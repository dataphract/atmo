use std::str::FromStr;

use serde::{de::Error as _, Deserialize, Serialize};

use crate::{did::Did, error::ParseError, handle::Handle, nsid::Nsid, rkey::RecordKey};

const MAX_LEN: usize = 8 * 1024;

/// An AT-URI.
pub struct AtUri {
    // The full text of the AT-URI.
    text: String,
    // Start index of the path segment.
    path_start: u16,
}

impl AtUri {
    const SCHEME: &[u8] = b"at://";

    pub fn new(uri: String) -> Option<AtUri> {
        if uri.len() > MAX_LEN {
            return None;
        }

        // Strip scheme.
        let bytes = uri.as_bytes();
        let bytes = bytes.strip_prefix(Self::SCHEME)?;

        let path_start = bytes.iter().position(|&c| c == b'/').unwrap_or(bytes.len());
        let (authority, path) = bytes.split_at(path_start);

        if authority.starts_with(b"did:") {
            Did::new(authority)?;
        } else {
            Handle::new(std::str::from_utf8(authority).unwrap())?;
        }

        // First item is the empty string before the leading slash.
        let mut components = path.split(|&c| c == b'/').skip(1);

        let Some(collection) = components.next() else {
            return Some(AtUri {
                text: uri,
                path_start: path_start as u16,
            });
        };

        if collection.is_empty() {
            return None;
        }

        Nsid::try_from(collection).ok()?;

        let Some(rkey) = components.next() else {
            return Some(AtUri {
                text: uri,
                path_start: path_start as u16,
            });
        };

        if rkey.is_empty() {
            return None;
        }

        RecordKey::try_from(rkey).ok()?;

        Some(AtUri {
            text: uri,
            path_start: path_start as u16,
        })
    }

    pub fn authority(&self) -> &str {
        let start = Self::SCHEME.len();
        let end = usize::from(self.path_start);
        &self.text[start..end]
    }

    pub fn path(&self) -> Option<&str> {
        let start = usize::from(self.path_start);
        (start != self.text.len()).then_some(&self.text[start..])
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

pub struct ParseAtUriError {
    _dummy: (),
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
