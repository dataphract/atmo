//! A Rust SDK for the [AT Protocol].
//!
//! [AT Protocol]: https://atproto.com/

#[cfg(doctest)]
#[doc = include_str!("../../README.md")]
pub struct ReadmeDoctest;

#[doc(inline)]
pub use atmo_api as api;

#[doc(inline)]
pub use atmo_core as core;

#[cfg(feature = "jetstream")]
#[doc(inline)]
pub use atmo_jetstream as jetstream;
