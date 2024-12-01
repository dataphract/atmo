use std::{borrow::Cow, fmt, str::FromStr};

use serde::Serialize;

use crate::{
    error::ParseError, impl_deserialize_via_from_str, is_valid_domain_segment, is_valid_nsid_name,
    is_valid_tld, SEGMENT_LEN_RANGE,
};

const MAX_LEN: usize = 317;
const MAX_AUTHORITY_LEN: usize = 253;
const MIN_SEGMENTS: usize = 3;

/// An ATProto namespaced identifier, or NSID.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nsid(Cow<'static, str>);

impl Nsid {
    #[doc(hidden)]
    pub const fn from_static_unchecked(s: &'static str) -> Self {
        Nsid(Cow::Borrowed(s))
    }

    /// Returns the NSID, viewed as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    pub fn name(&self) -> &str {
        self.segments().next_back().unwrap()
    }

    /// Returns an iterator over the segments of the NSID.
    pub fn segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.0.split('.')
    }

    pub fn with_fragment(&self, fragment: &Fragment) -> FullReference {
        FullReference::from_str(&format!("{self}{fragment}")).unwrap()
    }
}

impl fmt::Display for Nsid {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Nsid {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_nsid(s.as_bytes())?;

        Ok(Nsid(String::from(s).into()))
    }
}

impl TryFrom<&'_ [u8]> for Nsid {
    type Error = ParseError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        validate_nsid(bytes)?;

        let string = String::from_utf8(bytes.into()).unwrap();
        Ok(Nsid(string.into()))
    }
}

impl_deserialize_via_from_str!(Nsid);

impl Serialize for Nsid {
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(ser)
    }
}

fn validate_nsid(bytes: &[u8]) -> Result<(), ParseError> {
    if bytes.len() > MAX_LEN {
        return Err(ParseError::nsid());
    }

    let mut it = bytes.split(|&b| b == b'.').peekable();

    let tld = it.next().ok_or_else(ParseError::nsid)?;

    if !is_valid_tld(tld) {
        return Err(ParseError::nsid());
    }

    let mut len = tld.len();
    let mut num_segments = 1;
    while let Some(segment) = it.next() {
        let is_valid = match it.peek() {
            Some(_) => is_valid_domain_segment(segment),
            None => len < MAX_AUTHORITY_LEN && is_valid_nsid_name(segment),
        };

        num_segments += 1;
        len += 1 + segment.len();

        if !is_valid {
            return Err(ParseError::nsid());
        }
    }

    if num_segments < MIN_SEGMENTS {
        return Err(ParseError::nsid());
    }

    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fragment(String);

impl Fragment {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for Fragment {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_fragment(s.as_bytes()).map(|()| Fragment(s.into()))
    }
}

fn validate_fragment(bytes: &[u8]) -> Result<(), ParseError> {
    let bytes = bytes
        .strip_prefix(b"#")
        .ok_or_else(ParseError::nsid_fragment)?;

    if !SEGMENT_LEN_RANGE.contains(&bytes.len()) {
        return Err(ParseError::nsid_fragment());
    }

    if !bytes.iter().all(|c| c.is_ascii_alphanumeric()) {
        return Err(ParseError::nsid_fragment());
    }

    Ok(())
}

impl fmt::Display for Fragment {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

/// An NSID reference.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Reference {
    Full(FullReference),
    Relative(Fragment),
}

impl FromStr for Reference {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') {
            Fragment::from_str(s).map(Reference::Relative)
        } else {
            FullReference::from_str(s).map(Reference::Full)
        }
    }
}

impl fmt::Display for Reference {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reference::Full(r) => fmt::Display::fmt(r, f),
            Reference::Relative(r) => fmt::Display::fmt(r, f),
        }
    }
}

/// A fully-qualified NSID reference.
///
/// This consists of an NSID and an optional fragment.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FullReference {
    text: String,
    frag_start: usize,
}

impl FullReference {
    pub fn clone_nsid(&self) -> Nsid {
        Nsid(self.text[..self.frag_start].to_string().into())
    }

    #[inline]
    fn has_fragment(&self) -> bool {
        self.frag_start < self.text.len()
    }

    pub fn clone_fragment(&self) -> Option<Fragment> {
        self.has_fragment()
            .then(|| Fragment(self.text[self.frag_start..].to_string()))
    }

    pub fn fragment_name(&self) -> Option<&str> {
        self.has_fragment()
            .then(|| &self.text[self.frag_start + 1..])
    }
}

impl From<Nsid> for FullReference {
    #[inline]
    fn from(nsid: Nsid) -> Self {
        FullReference {
            frag_start: nsid.0.len(),
            text: nsid.to_string(),
        }
    }
}

impl FromStr for FullReference {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let frag_start = s.find('#').unwrap_or(s.len());
        let (nsid_s, frag_s) = s.split_at(frag_start);

        validate_nsid(nsid_s.as_bytes())?;

        if !frag_s.is_empty() {
            validate_fragment(frag_s.as_bytes())?;
        }

        Ok(FullReference {
            text: s.into(),
            frag_start,
        })
    }
}

impl fmt::Display for FullReference {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.text.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_examples() {
        crate::test::test_valid::<Nsid>([
            "com.example.fooBar",
            "net.users.bob.ping",
            "a-0.b-1.c",
            "a.b.c",
            "cn.8.lex.stuff",
        ]);
    }

    #[test]
    fn invalid_examples() {
        crate::test::test_invalid::<Nsid>(["com.exa🤯ple.thing", "com.example"]);
    }
}
