use std::ops::Range;

#[inline]
pub fn is_uri_unreserved(c: u8) -> bool {
    c.is_ascii_alphanumeric() || b".-_~".contains(&c)
}

// Taken from stdlib until stabilized.
#[inline]
pub fn subslice_range(this: &[u8], subslice: &[u8]) -> Option<Range<usize>> {
    let this_start = this.as_ptr() as usize;
    let subslice_start = subslice.as_ptr() as usize;

    let start = subslice_start.wrapping_sub(this_start);

    // NOTE: Elided, since size_of::<u8>() is 1.
    //
    // if byte_start % core::mem::size_of::<T>() != 0 {
    //     return None;
    // }
    //
    // let start = byte_start / core::mem::size_of::<T>();

    let end = start.wrapping_add(subslice.len());

    if start <= this.len() && end <= this.len() {
        Some(start..end)
    } else {
        None
    }
}
