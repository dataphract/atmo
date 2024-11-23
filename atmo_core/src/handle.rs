use std::{ops::RangeInclusive, str::FromStr};

use serde::Serialize;

use crate::{
    error::ParseError, impl_deserialize_via_from_str, is_valid_domain_segment, is_valid_tld,
};

const LEN_RANGE: RangeInclusive<usize> = 1..=253;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Handle(String);

impl Handle {
    pub fn new(handle: &str) -> Option<Handle> {
        validate_handle(handle.as_bytes()).ok()?;
        Some(Handle(handle.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl_deserialize_via_from_str!(Handle);

impl Serialize for Handle {
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(ser)
    }
}

fn validate_handle(bytes: &[u8]) -> Result<(), ParseError> {
    if !LEN_RANGE.contains(&bytes.len()) {
        return Err(ParseError::handle());
    }

    let mut it = bytes.split(|&b| b == b'.').peekable();

    let mut num_segments = 0;
    while let Some(segment) = it.next() {
        num_segments += 1;

        let is_valid = match it.peek() {
            Some(_) => is_valid_domain_segment(segment),
            None => is_valid_tld(segment),
        };

        if !is_valid {
            return Err(ParseError::handle());
        }
    }

    if num_segments < 2 {
        return Err(ParseError::handle());
    }

    Ok(())
}

impl FromStr for Handle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_handle(s.as_bytes()).map(|()| Handle(s.into()))
    }
}

impl TryFrom<&[u8]> for Handle {
    type Error = ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        validate_handle(bytes).map(|()| Handle(std::str::from_utf8(bytes).unwrap().into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! valid {
        ($($input:literal),* $(,)?) => {
            $(
                if Handle::new($input.into()).is_none() {
                    panic!("valid Handle rejected: {}", $input);
                }
            )*
        };
    }

    macro_rules! invalid {
        ($($input:literal),* $(,)?) => {
            $(
                if Handle::new($input.into()).is_some() {
                    panic!("invalid Handle accepted: {}", $input);
                }
            )*
        };
    }

    #[test]
    fn valid_examples() {
        valid![
            "jay.bsky.social",
            "8.cn",
            "name.t--t",
            "XX.LCS.MIT.EDU",
            "a.co",
            "xn--notarealidn.com",
            "xn--fiqa61au8b7zsevnm8ak20mc4a87e.xn--fiqs8s",
            "xn--ls8h.test",
            "example.t",
        ];
    }

    #[test]
    fn invalid_examples() {
        invalid![
            "",
            "jo@hn.test",
            "💩.test",
            "john..test",
            "xn--bcher-.tld",
            "john.0",
            "cn.8",
            "www.masełkowski.pl.com",
            "org",
            "name.org.",
        ];
    }
}
