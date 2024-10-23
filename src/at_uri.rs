use std::ops::Range;

use crate::split_once;

const MAX_LEN: usize = 8 * 1024;

pub struct AtUri {
    // The full text of the AT-URI.
    text: String,
    // Byte range of the authority segment.
    authority: Slice,
    // Byte range of the path segment.
    path: Slice,
}

impl AtUri {
    pub fn new(uri: String) -> Option<AtUri> {
        let bytes = uri.as_bytes();
        let bytes = bytes.strip_prefix(b"at://")?;
        let (_authority, _rest) = split_once(bytes, |&c| c == b'/')?;

        todo!()
    }

    fn slice(&self, slice: Slice) -> &str {
        &self.text[slice.range()]
    }

    pub fn authority(&self) -> &str {
        self.slice(self.authority)
    }
}

#[derive(Copy, Clone)]
struct Slice {
    start: u16,
    end: u16,
}

impl Slice {
    fn range(&self) -> Range<usize> {
        self.start.into()..self.end.into()
    }
}
