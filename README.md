# atmo

a Rust library for the [AT Protocol]

## status

**This is a work in progress.**

At present, there are three crates:

- `atmo` contains the core API types (AT-URI, CID, DID, etc).
- `atmo_codegen` contains the bindings generator.
- `atmo_lexicon` contains the Lexicon schema parser.

Once the code generator is complete, the generated bindings will be committed as another crate.
More will follow. Stay tuned!

## usage

To generate bindings (which are probably broken, at time of writing) for a set of Lexicon schemae,
run

``` sh
cargo run --release -p atmo_codegen --bin gen -- <DIR> | rustfmt > bindings.rs
```

...where `<DIR>` is the root of the schema tree. The code generator is being tested against the
[atproto Lexicons]; I can't guarantee that it'll work against anything else.

[AT Protocol]: https://atproto.com
[atproto Lexicons]: https://github.com/bluesky-social/atproto/tree/main/lexicons
