# Atmo

A Rust SDK for the [AT Protocol].

## Overview

Atmo provides high-level clients for [XRPC] and [Jetstream] via the `atmo` and `atmo_jetstream`
crates. These crates use the bindings provided by `atmo_api`, which are parsed from the
[ATProto Lexicons] using `atmo_lexicon` and generated by `atmo_codegen`. All these crates depend
on `atmo_core`, which implements the core of the ATProto data model.

[XRPC]: https://atproto.com/specs/xrpc
[AT Protocol]: https://atproto.com
[ATProto Lexicons]: https://github.com/bluesky-social/atproto/tree/main/lexicons
[Jetstream]: https://github.com/bluesky-social/jetstream
