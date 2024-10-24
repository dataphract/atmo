#[inline]
pub fn is_uri_unreserved(c: u8) -> bool {
    c.is_ascii_alphanumeric() || b".-_~".contains(&c)
}
