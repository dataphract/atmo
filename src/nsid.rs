use std::{fmt, str::FromStr};

use crate::{error::ParseError, is_valid_domain_segment, is_valid_nsid_name, is_valid_tld};

const MAX_LEN: usize = 317;
const MAX_AUTHORITY_LEN: usize = 253;
const MIN_SEGMENTS: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nsid(String);

impl Nsid {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn segments(&self) -> impl DoubleEndedIterator<Item = &str> {
        self.0.split('.')
    }
}

impl fmt::Display for Nsid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl FromStr for Nsid {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_nsid(s.as_bytes()).map(|()| Nsid(s.into()))
    }
}

impl TryFrom<&'_ [u8]> for Nsid {
    type Error = ParseError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        validate_nsid(bytes).map(|()| Nsid(String::from_utf8(bytes.into()).unwrap()))
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
