use std::{fmt, ops::RangeInclusive, str::FromStr};

const LEN_RANGE: RangeInclusive<usize> = 1..=512;

pub struct RecordKey(String);

impl FromStr for RecordKey {
    type Err = ParseRecordKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !LEN_RANGE.contains(&s.len()) {
            return Err(ParseRecordKeyError::new());
        }

        if s == "." || s == ".." {
            return Err(ParseRecordKeyError::new());
        }

        s.chars()
            .all(|c| c.is_ascii_alphanumeric() || ".-_:~".contains(c))
            .then(|| RecordKey(s.into()))
            .ok_or_else(ParseRecordKeyError::new)
    }
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
