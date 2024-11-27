use std::{fmt, num::NonZeroUsize, str::FromStr};

use percent_encoding::percent_encode;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error::ParseError,
    impl_deserialize_via_from_str,
    parse::{is_uri_sub_delim, is_uri_unreserved, PCT_FRAGMENT_SET, PCT_QUERY_SET},
    split_once, Handle,
};

/// A Decentralized Identifier, or DID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(test, derive(proptest_derive::Arbitrary))]
pub struct Did(
    #[cfg_attr(
        test,
        proptest(
            regex = r#"did:[a-z]+:([0-9A-Za-z._:-]|%[[:xdigit:]]{2})*([a-zA-Z0-9._-]|%[[:xdigit:]]{2})"#
        )
    )]
    String,
);

impl Did {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn method(&self) -> &str {
        self.0
            .split(':')
            .nth(1)
            .expect("DID missing method, should have been validated already")
    }
}

impl FromStr for Did {
    type Err = ParseError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        validate_did(s.as_bytes())?;

        Ok(Did(s.into()))
    }
}

impl TryFrom<&[u8]> for Did {
    type Error = ParseError;

    #[inline]
    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        validate_did(bytes)?;

        Ok(Did(String::from_utf8(bytes.into()).unwrap()))
    }
}

impl_deserialize_via_from_str!(Did);

impl Serialize for Did {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(ser)
    }
}

fn validate_did(input: &[u8]) -> Result<(), ParseError> {
    let Some(input) = input.strip_prefix(b"did:") else {
        return Err(ParseError::did());
    };

    let Some((method, ident)) = split_once(input, |&c| c == b':') else {
        return Err(ParseError::did());
    };

    if !is_valid_did_method(method) || !is_valid_did_ident(ident) {
        return Err(ParseError::did());
    }

    Ok(())
}

#[inline]
fn is_valid_did_method(input: &[u8]) -> bool {
    !input.is_empty() && input.iter().all(|b| b.is_ascii_lowercase())
}

fn is_valid_did_ident(input: &[u8]) -> bool {
    if input.is_empty() {
        return false;
    }

    let mut it = input.iter().copied().peekable();

    while let Some(c) = it.next() {
        match c {
            // Accept.
            b'.' | b'_' | b'-' => (),
            a if a.is_ascii_alphanumeric() => (),

            // Identifier cannot end with a colon.
            b':' => {
                if it.peek().is_none() {
                    return false;
                }
            }

            // Percent must be followed by two hex digits.
            b'%' => {
                if !(it.next().is_some_and(|c| c.is_ascii_hexdigit())
                    && it.next().is_some_and(|c| c.is_ascii_hexdigit()))
                {
                    return false;
                }
            }

            // Reject anything else.
            _ => return false,
        }
    }

    true
}

#[derive(Clone, Debug)]
pub struct DidUrl {
    did: Did,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}

impl FromStr for DidUrl {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        let did_end = bytes
            .iter()
            .position(|&c| c == b'/' || c == b'?' || c == b'#')
            .unwrap_or(bytes.len());

        let (did, rest) = bytes.split_at(did_end);

        let did = Did::try_from(did)?;

        let (path, rest) = if rest.starts_with(b"/") {
            let path_end = rest
                .iter()
                .position(|&c| c == b'?' || c == b'#')
                .unwrap_or(rest.len());

            let (path, rest) = rest.split_at(path_end);

            validate_did_path(path)?;

            let path = std::str::from_utf8(path).unwrap().to_owned();

            (Some(path), rest)
        } else {
            (None, rest)
        };

        let (query, rest) = if rest.starts_with(b"?") {
            let query_end = rest.iter().position(|&c| c == b'#').unwrap_or(rest.len());

            let (query, rest) = rest.split_at(query_end);

            validate_did_query(query)?;

            let query = std::str::from_utf8(query).unwrap().to_owned();

            (Some(query), rest)
        } else {
            (None, rest)
        };

        let fragment = if rest.starts_with(b"#") {
            validate_did_fragment(rest)?;

            let fragment = std::str::from_utf8(rest).unwrap().to_owned();

            Some(fragment)
        } else {
            None
        };

        Ok(DidUrl {
            did,
            path,
            query,
            fragment,
        })
    }
}

impl fmt::Display for DidUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.did.as_str())?;

        if let Some(p) = &self.path {
            f.write_str(p)?
        }

        if let Some(q) = &self.query {
            write!(f, "{}", percent_encode(q.as_bytes(), &PCT_QUERY_SET))?;
        }

        if let Some(frag) = &self.fragment {
            write!(f, "{}", percent_encode(frag.as_bytes(), &PCT_FRAGMENT_SET))?;
        }

        Ok(())
    }
}

impl_deserialize_via_from_str!(DidUrl);

impl Serialize for DidUrl {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO(dp): don't do this
        let s = self.to_string();

        s.serialize(ser)
    }
}

fn validate_did_path(path: &[u8]) -> Result<(), ParseError> {
    let mut rest = path;

    while !rest.is_empty() {
        rest = rest.strip_prefix(b"/").ok_or(ParseError::did_url())?;

        while let Some(len) = match_path_char(rest) {
            rest = &rest[len.get()..];
        }
    }

    Ok(())
}

fn match_path_char(segment: &[u8]) -> Option<NonZeroUsize> {
    let (&c, rest) = segment.split_first()?;

    if is_uri_unreserved(c) || is_uri_sub_delim(c) || c == b':' || c == b'@' {
        return NonZeroUsize::new(1);
    }

    if c != b'%' {
        return None;
    }

    let (code, _rest) = rest.split_at_checked(2)?;

    code.iter()
        .all(u8::is_ascii_hexdigit)
        .then_some(NonZeroUsize::new(3).unwrap())
}

fn validate_did_query(query: &[u8]) -> Result<(), ParseError> {
    let rest = query.strip_prefix(b"?").ok_or(ParseError::did_url())?;

    validate_did_query_or_fragment_tail(rest)
}

fn validate_did_fragment(query: &[u8]) -> Result<(), ParseError> {
    let rest = query.strip_prefix(b"#").ok_or(ParseError::did_url())?;

    validate_did_query_or_fragment_tail(rest)
}

fn validate_did_query_or_fragment_tail(mut rest: &[u8]) -> Result<(), ParseError> {
    while let Some(len) = match_query_or_fragment_char(rest) {
        rest = &rest[len.get()..];
    }

    if !rest.is_empty() {
        return Err(ParseError::did_url());
    }

    Ok(())
}

fn match_query_or_fragment_char(input: &[u8]) -> Option<NonZeroUsize> {
    let &c = input.first()?;

    if c == b'/' || c == b'?' {
        return NonZeroUsize::new(1);
    }

    match_path_char(input)
}

/// A DID document.
///
/// This type represents the subset of DID documents recognized by ATProto.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DidDoc {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: Did,
    pub also_known_as: Vec<String>,
    pub verification_method: Option<Vec<VerificationMethod>>,
    pub service: Vec<Service>,
}

impl DidDoc {
    pub fn handle(&self) -> Option<Handle> {
        self.also_known_as
            .iter()
            .find_map(|s| Handle::from_str(s).ok())
    }

    pub fn pds_service_url(&self) -> Option<&Url> {
        self.service.iter().find_map(|svc| {
            (svc.ty == "AtprotoPersonalDataServer" && svc.id.ends_with("#atproto_pds"))
                .then_some(&svc.service_endpoint)
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationMethod {
    pub id: DidUrl,
    pub controller: Did,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub service_endpoint: Url,
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use crate::test::{test_invalid, test_valid};

    use super::*;

    #[test]
    fn valid_examples() {
        test_valid::<Did>([
            "did:plc:z72i7hdynmk6r22z27h6tvur",
            "did:web:blueskyweb.xyz",
            "did:method:val:two",
            "did:m:v",
            "did:method::::val",
            "did:method:-:_:.",
            "did:key:zQ3shZc2QzApp2oymGvQbzP8eKheVshBHbU4ZYjeXqwSKEn6N",
        ])
    }

    #[test]
    fn invalid_examples() {
        test_invalid::<Did>([
            "missing:scheme", // Missing scheme
            "DID:method:val", // Uppercase scheme
            "did::val",       // Missing method
            "did:UPPER:val",  // Uppercase method
            "did:m123:val",
            "did:method:",
            "did:method:val/two",
            "did:method:val?two",
            "did:method:val#two",
        ]);
    }

    proptest! {
        #[test]
        fn proptest_did_roundtrip(did: Did) {
            let serialized = serde_json::to_string(&did).unwrap();
            let deserialized: Did = serde_json::from_str(&serialized).unwrap();

            assert_eq!(did, deserialized);
        }
    }
}
