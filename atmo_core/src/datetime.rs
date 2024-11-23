use std::{cmp::Ordering, fmt, str::FromStr, sync::LazyLock};

use jiff::{
    fmt::{strtime::BrokenDownTime, temporal::DateTimePrinter},
    tz::Offset,
    Timestamp,
};
use serde::{de::Error, Deserialize, Serialize};

use crate::error::ParseError;

const PRINT_SUBSEC_PRECISION: u8 = 6;
const STORED_SUBSEC_PRECISION: usize = 9;

static PRINTER: LazyLock<DateTimePrinter> =
    LazyLock::new(|| DateTimePrinter::new().precision(Some(PRINT_SUBSEC_PRECISION)));

/// A parsed string representing a `DateTime`.
///
/// # Serialization and Deserialization
///
/// This type allows lossless round-trip deserialization and serialization. Per the
/// [ATProto specification](https://atproto.com/specs/lexicon#string-formats):
///
/// > _Implementations should be aware when round-tripping records containing datetimes of two_
/// > _ambiguities: loss-of-precision, and ambiguity with trailing fractional second zeros. [...]_
/// > _A safer thing to do is to deserialize the datetime as a simple string, which ensures_
/// > _round-trip re-serialization._
///
/// To that end, this type stores the original parsed string and exposes it via
/// [`DateTimeString::as_str`].
///
/// # Comparison
///
/// The `Eq` implementation for this type compares the underlying `String` representation, rather
/// than comparing the timestamp. As such, two `DateTimeStrings` representing the same numeric date
/// and time may compare unequal.
#[derive(Clone, Debug)]
pub struct DateTimeString {
    // Keep the original representation to allow round-tripping, hashing, etc.
    original: String,
    parsed: Timestamp,
}

impl PartialEq for DateTimeString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original
    }
}

impl Eq for DateTimeString {}

impl DateTimeString {
    pub fn from_unix_seconds(seconds: i64) -> Option<Self> {
        let parsed = jiff::Timestamp::from_second(seconds).ok()?;

        let original = PRINTER.timestamp_to_string(&parsed);
        Some(DateTimeString { original, parsed })
    }

    /// Returns the string representation of the `DateTime` as originally parsed.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.original.as_str()
    }

    #[inline]
    pub fn timestamp(&self) -> jiff::Timestamp {
        self.parsed
    }
}

impl From<jiff::Timestamp> for DateTimeString {
    #[inline]
    fn from(timestamp: jiff::Timestamp) -> Self {
        DateTimeString {
            original: PRINTER.timestamp_to_string(&timestamp),
            parsed: timestamp,
        }
    }
}

impl FromStr for DateTimeString {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let original = s.to_owned();
        let parsed = parse(s).ok_or_else(ParseError::datetime)?;

        Ok(DateTimeString { original, parsed })
    }
}

impl<'de> Deserialize<'de> for DateTimeString {
    fn deserialize<D>(des: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let original = String::deserialize(des)?;
        let parsed = parse(&original).ok_or_else(|| D::Error::custom("invalid DateTime string"))?;

        Ok(DateTimeString { original, parsed })
    }
}

impl Serialize for DateTimeString {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.original.serialize(ser)
    }
}

fn parse(input: &str) -> Option<Timestamp> {
    let mut time = BrokenDownTime::default();

    // All DateTimes are required to start with the standard `yyyy-mm-ddThh:mm:ss`.

    // Year.
    let (year, input) = input.split_at_checked(4)?;
    let year = year.parse().ok()?;
    time.set_year(Some(year)).ok()?;

    let input = input.strip_prefix('-')?;

    // Month.
    let (month, input) = input.split_at_checked(2)?;
    let month = month.parse().ok()?;
    time.set_month(Some(month)).ok()?;

    let input = input.strip_prefix('-')?;

    // Day.
    let (day, input) = input.split_at_checked(2)?;
    let day = day.parse().ok()?;
    time.set_day(Some(day)).ok()?;

    let input = input.strip_prefix('T')?;

    // Hour.
    let (hour, input) = input.split_at_checked(2)?;
    let hour = hour.parse().ok()?;
    time.set_hour(Some(hour)).ok()?;

    let input = input.strip_prefix(':')?;

    // Minute.
    let (minute, input) = input.split_at_checked(2)?;
    let minute = minute.parse().ok()?;
    time.set_minute(Some(minute)).ok()?;

    let input = input.strip_prefix(':')?;

    // Second.
    let (second, input) = input.split_at_checked(2)?;
    let second = second.parse().ok()?;
    let leap_second = second == 60;

    // If there's a leap second, clamp to 59.999999999 seconds.
    let second = if leap_second { 59 } else { second };

    time.set_second(Some(second)).ok()?;

    // Subseconds are optional, with arbitrary precision.
    let (subsec_nanos, input) = 'subsec: {
        match input.strip_prefix('.') {
            Some(input) => {
                let nondigit = input.find(|c: char| !c.is_ascii_digit())?;
                let (digits, input) = input.split_at_checked(nondigit)?;

                // RFC 3339 requires that at least one digit occur if there is a decimal point.
                if digits.is_empty() {
                    return None;
                }

                // If the timestamp occurs during a leap second, skip parsing the subseconds.
                if leap_second {
                    break 'subsec (999999999, input);
                }

                // Safe cast: String cannot be longer than isize::MAX.
                let relative_precision = STORED_SUBSEC_PRECISION as isize - digits.len() as isize;

                match relative_precision.cmp(&0) {
                    Ordering::Less => {
                        let (nanos, _truncated) =
                            digits.split_at_checked(STORED_SUBSEC_PRECISION)?;
                        let nanos = nanos.parse().ok()?;

                        (nanos, input)
                    }

                    Ordering::Equal => (digits.parse().ok()?, input),

                    Ordering::Greater => {
                        // Safe cast: value is 1..9
                        let order = relative_precision as u32;
                        let nanos = digits.parse::<i32>().ok().map(|n| n * 10i32.pow(order))?;

                        (nanos, input)
                    }
                }
            }

            None => (0, input),
        }
    };

    time.set_subsec_nanosecond(Some(subsec_nanos)).ok()?;

    // Offset specification is required, either with `Z` (for UTC) or [+|-]hh:mm.
    // `-00:00` is disallowed.

    let (zone_start, input) = {
        let mut it = input.chars();
        (it.next(), it.as_str())
    };

    let offset = match (zone_start, input) {
        (Some('Z'), "") => Offset::UTC,
        (Some('Z'), _trailing) => {
            return None;
        }
        (Some(sign @ '+'), input) | (Some(sign @ '-'), input) => {
            let (offset_hours, input) = input.split_at_checked(2)?;
            let offset_hours: i32 = offset_hours.parse().ok()?;

            let input = input.strip_prefix(':')?;

            let (offset_minutes, input) = input.split_at_checked(2)?;
            let offset_minutes: i32 = offset_minutes.parse().ok()?;

            // Trailing characters are not allowed.
            if !input.is_empty() {
                return None;
            }

            let offset_seconds = 3600 * offset_hours + 60 * offset_minutes;

            // Offset of -00:00 is not allowed.
            if sign == '-' && offset_seconds == 0 {
                return None;
            }

            Offset::from_seconds(offset_seconds).ok()?
        }
        _ => {
            return None;
        }
    };

    time.set_offset(Some(offset));

    time.to_timestamp().ok()
}

#[derive(Debug)]
pub struct ParseDateTimeError {
    // Dummy field to allow fields to be added without breaking semver.
    _dummy: (),
}

impl fmt::Display for ParseDateTimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid DateTime")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_examples() {
        crate::test::test_valid::<DateTimeString>([
            // valid
            "1985-04-12T23:20:50.123Z",
            "1985-04-12T23:20:50.123456Z",
            "1985-04-12T23:20:50.120Z",
            "1985-04-12T23:20:50.120000Z",
            // supported
            "1985-04-12T23:20:50.12345678912345Z",
            "1985-04-12T23:20:50Z",
            "1985-04-12T23:20:50.0Z",
            "1985-04-12T23:20:50.123+00:00",
            "1985-04-12T23:20:50.123-07:00",
            // allow leap seconds
            "2024-12-31T23:59:60.123456789Z",
        ]);
    }

    #[test]
    fn invalid_examples() {
        crate::test::test_invalid::<DateTimeString>([
            "1985-04-12",
            "1985-04-12T23:20Z",
            "1985-04-12T23:20:5Z",
            "1985-04-12T23:20:50.123",
            "+001985-04-12T23:20:50.123Z",
            "23:20:50.123Z",
            "-1985-04-12T23:20:50.123Z",
            "1985-4-12T23:20:50.123Z",
            "01985-04-12T23:20:50.123Z",
            "1985-04-12T23:20:50.123+00",
            "1985-04-12T23:20:50.123+0000",
            // ISO-8601 strict capitalization
            "1985-04-12t23:20:50.123Z",
            "1985-04-12T23:20:50.123z",
            // RFC-3339, but not ISO-8601
            "1985-04-12T23:20:50.123-00:00",
            "1985-04-12 23:20:50.123Z",
            // timezone is required
            "1985-04-12T23:20:50.123",
            // syntax looks ok, but datetime is not valid
            "1985-04-12T23:99:50.123Z",
            "1985-00-12T23:20:50.123Z",
            // too many days in february
            "2024-02-30T12:12:12.12345Z",
        ]);
    }
}
