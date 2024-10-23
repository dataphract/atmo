use std::fmt;

use crate::{is_valid_domain_segment, is_valid_nsid_name, is_valid_tld};

const MAX_LEN: usize = 317;
const MAX_AUTHORITY_LEN: usize = 253;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nsid(String);

impl Nsid {
    pub fn new(nsid: String) -> Option<Nsid> {
        if nsid.len() > MAX_LEN {
            return None;
        }

        // Avoid UTF-8 code paths.
        let bytes = nsid.as_bytes();

        let mut it = bytes.split(|&b| b == b'.').peekable();

        let tld = it.next()?;

        if !is_valid_tld(tld) {
            return None;
        }

        let mut len = tld.len();
        while let Some(segment) = it.next() {
            let is_valid = match it.peek() {
                Some(_) => is_valid_domain_segment(segment),
                None => len < MAX_AUTHORITY_LEN && is_valid_nsid_name(segment),
            };

            len += 1 + segment.len();

            if !is_valid {
                return None;
            }
        }

        Some(Nsid(nsid))
    }

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
