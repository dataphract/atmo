use std::{fmt, ops::RangeInclusive, str::FromStr};

use crate::impl_deserialize_via_from_str;

const LEN_RANGE: RangeInclusive<usize> = 1..=512;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RecordKey(String);

impl RecordKey {
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for RecordKey {
    type Err = ParseRecordKeyError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        is_valid_record_key(s.as_bytes())
            .then(|| RecordKey(s.into()))
            .ok_or_else(ParseRecordKeyError::new)
    }
}

impl_deserialize_via_from_str!(RecordKey);

impl TryFrom<&'_ [u8]> for RecordKey {
    type Error = ParseRecordKeyError;

    #[inline]
    fn try_from(bytes: &'_ [u8]) -> Result<Self, Self::Error> {
        is_valid_record_key(bytes)
            .then(|| RecordKey(String::from_utf8(bytes.into()).unwrap()))
            .ok_or_else(ParseRecordKeyError::new)
    }
}

fn is_valid_record_key(bytes: &[u8]) -> bool {
    LEN_RANGE.contains(&bytes.len())
        && bytes != b"."
        && bytes != b".."
        && bytes
            .iter()
            .copied()
            .all(|c| crate::parse::is_uri_unreserved(c) || c == b':')
}

#[derive(Debug)]
pub struct ParseRecordKeyError {
    _dummy: (),
}

impl ParseRecordKeyError {
    fn new() -> Self {
        Self { _dummy: () }
    }
}

impl fmt::Display for ParseRecordKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid RecordKey string")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_examples() {
        crate::test::test_valid::<RecordKey>([
            "3jui7kd54zh2y",
            "self",
            "example.com",
            "~1.2-3_",
            "dHJ1ZQ",
            "pre:fix",
            "_",
        ]);
    }

    #[test]
    fn invalid_examples() {
        crate::test::test_invalid::<RecordKey>([
            "alpha/beta",
            ".",
            "..",
            "#extra",
            "@handle",
            "any space",
            "any+space",
            "number[3]",
            "number(3)",
            "\"quote\"",
            "dHJ1ZQ==",
        ]);
    }
}
