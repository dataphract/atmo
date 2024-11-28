use std::{
    fmt::{self, Write},
    str::FromStr,
};

use serde::Serialize;

use crate::{impl_deserialize_via_from_str, ParseError};

const CHARSET: &[u8; 32] = b"234567abcdefghijklmnopqrstuvwxyz";
const LEN: usize = 13;

#[rustfmt::skip]
const LUT: [u8; 256] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11, 0x12, 0x13, 0x14,
    0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

/// A timestamp identifier.
///
/// The TID format is described by the ATProto [Record Key] specification.
///
/// [Record Key]: https://atproto.com/specs/record-key
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Tid(#[cfg_attr(test, proptest(strategy = "Tid::ZERO.0..=Tid::MAX.0"))] u64);

impl Tid {
    #[cfg(test)]
    const ZERO: Self = Tid(0);
    #[cfg(test)]
    const MAX: Self = Tid((1 << 63) - 1);

    /// Constructs a new `Tid` given a microsecond timestamp and a clock identifier.
    ///
    /// The timestamp value must be less than 2<sup>53</sup>, and the clock ID must be less than
    /// 2<sup>10</sup>. If either value is out of range, this function returns `None`.
    pub fn new(unix_timestamp_micros: u64, clock_id: u16) -> Option<Tid> {
        (unix_timestamp_micros < (1 << 53) && clock_id < (1 << 10))
            .then_some(Tid(unix_timestamp_micros << 10 | u64::from(clock_id)))
    }

    /// Constructs a `Tid` from the bits of a `u64`.
    ///
    /// If `bits` is not a valid `Tid` value, returns `None`.
    pub fn from_bits(bits: u64) -> Option<Tid> {
        (bits < (1 << 63)).then_some(Tid(bits))
    }

    /// Returns the encoded clock identifier.
    pub fn clock_id(self) -> u16 {
        (self.0 & ((1 << 10) - 1)) as u16
    }

    /// Returns the encoded microsecond timestamp.
    pub fn unix_timestamp_micros(self) -> u64 {
        self.0 >> 10
    }
}

impl FromStr for Tid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Tid::try_from(s.as_bytes())
    }
}

impl TryFrom<&[u8]> for Tid {
    type Error = ParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != LEN {
            return Err(ParseError::tid());
        }

        let mut it = bytes.iter().copied();
        let first = it.next().unwrap();

        if first > b'b' {
            return Err(ParseError::tid());
        }

        let mut value: u64 = LUT[first as usize].into();
        for byte in it {
            let bits = LUT[byte as usize];

            if bits > 0b11111 {
                return Err(ParseError::tid());
            }

            value = value << 5 | u64::from(bits);
        }

        Ok(Tid(value))
    }
}

impl fmt::Display for Tid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut shift = 60;

        for _ in 0..LEN {
            let byte = CHARSET[(self.0 as usize >> shift) & 0b11111];

            f.write_char(char::from(byte))?;

            shift -= 5;
        }

        Ok(())
    }
}

impl Serialize for Tid {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ser.collect_str(self)
    }
}

impl_deserialize_via_from_str!(Tid);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_examples() {
        crate::test::test_valid::<Tid>([
            "3jzfcijpj2z2a",
            "7777777777777",
            "3zzzzzzzzzzzz",
            "2222222222222",
        ]);
    }

    #[test]
    fn invalid_examples() {
        crate::test::test_invalid::<Tid>([
            // not base32
            "3jzfcijpj2z21",
            "0000000000000",
            // case-sensitive
            "3JZFCIJPJ2Z2A",
            // too long/short
            "3jzfcijpj2z2aa",
            "3jzfcijpj2z2",
            "222",
            // legacy dash syntax *not* supported (TTTT-TTT-TTTT-CC)
            "3jzf-cij-pj2z-2a",
            // high bit can't be set
            "zzzzzzzzzzzzz",
            "kjzfcijpj2z2a",
        ]);
    }

    #[test]
    fn limits() {
        assert_eq!(Tid::from_str("2222222222222").unwrap(), Tid::ZERO);
        assert_eq!(Tid::from_str("bzzzzzzzzzzzz").unwrap(), Tid::MAX);
        assert!(Tid::from_str("c222222222222").is_err());
        assert!(Tid::from_str("czzzzzzzzzzzz").is_err());
    }

    #[test]
    fn display() {
        assert_eq!(Tid::ZERO.to_string(), "2222222222222");
        assert_eq!(Tid::MAX.to_string(), "bzzzzzzzzzzzz");
    }

    proptest::proptest! {
        #[test]
        fn proptest_tid_roundtrip(tid: Tid) {
            let serialized = serde_json::to_string(&tid).unwrap();
            let deserialized = serde_json::from_str(&serialized).unwrap();
            assert_eq!(tid, deserialized);
        }
    }
}
