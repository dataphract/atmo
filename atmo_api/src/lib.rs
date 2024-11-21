//! Bindings to the ATProto API.
//!
//! These bindings are generated automatically based on the [atproto Lexicons].
//!
//! [atproto Lexicons]: https://github.com/bluesky-social/atproto/tree/main/lexicons
// TODO(dp): box large enum variants
#![allow(clippy::large_enum_variant)]

mod generated;
#[cfg(test)]
mod tests;

pub use generated::*;
