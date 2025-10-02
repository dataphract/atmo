//! Code generator for `atmo`.
//!
//! # Overview
//!
//! This crate contains a code generator which translates ATProto Lexicon files into equivalent Rust
//! definitions. The generated code is designed to be as ergonomic as possible without requiring
//! extensive manual intervention.
//!
//! # Module structure
//!
//! Rust modules are generated for each segment of an NSID which appears in the input Lexicon schemae.
//! For example, given schemae with these NSIDs as input:
//!
//! - `com.atproto.admin.getAccountInfo`
//! - `com.atproto.repo.applyWrites`
//!
//! The generator produces the following module structure:
//!
//! ```ignore
//! mod com {
//!     mod atproto {
//!         mod admin {
//!             mod get_account_info {}
//!         }
//!
//!         mod repo {
//!             mod apply_writes {}
//!         }
//!     }
//! }
//! ```
//!
//! # Placement of generated definitions
//!
//! In most cases, items defined by a Lexicon schema are emitted in the module corresponding to the
//! schema's NSID. For example, the Rust type generated for
//!
//! ```text
//! com.atproto.repo.listMissingBlobs#recordBlob
//! ```
//!
//! is emitted so that it can be imported as follows:
//!
//! ```ignore
//! use com::atproto::repo::list_missing_blobs::RecordBlob;
//! ```
//!
//! There are two special cases, in which items are emitted in the parent module instead:
//!
//! - The primary definition in a Lexicon schema, identified by the fragment `#main`
//! - Definitions in a Lexicon schema whose NSID name is `defs`
//!
//! # Mapping from Lexicon definitions to Rust types
//!
//! Todo...
//!
//! # Intended output
//!
//! The following rules dictate the intended output of the generator:
//!
//! - The NSID of a lexicon file determines the module(s) in which its constituent types are located.
//!   For example, all types and RPCs under the `com.atproto.actor.*` NSID should be located in
//!   the module `com::atproto::actor`, with the following exceptions:
//!
//!   - The main definition in a lexicon should be emitted one level higher than all other definitions
//!     in that lexicon. For example, the RPC `com.atproto.repo.applyWrites` should be emitted as
//!     `com::atproto::repo::ApplyWrites`, while the struct `com.atproto.repo.applyWrites#create`
//!     should be emitted as `com::atproto::repo::apply_writes::Create`.
//!
//!   - All items defined in an NSID with the name `defs` should be emitted in the parent module.
//!     The use of `defs` is a convention, so this behavior may need updating depending on how the
//!     ecosystem grows.
//!
//! - Type definitions should be emitted for the following lexicon items:
//!
//!   - Any `object` which appears in the schema should have an equivalent Rust `struct` emitted.
//!
//!     - If the object appears in the `defs` map, it should be emitted with the `PascalCase`
//!       equivalent of its def name.
//!
//!     - If the object appears as the `params`, `input`, or `output` field of a `query` or
//!       `procedure`, it should be emitted at `rpc_name::{Params, Input, Output}` as appropriate.
//!
//!     - If the object appears as the `record` field of a `record` definition, it should be emitted
//!       with the `PascalCase` equivalent of the record type name.

use atmo_lexicon::{Lexicon, StringFormat};
use namespace::{BuiltinTy, NamespaceTree};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::module::ItemPath;

mod enum_;
mod module;
mod namespace;
mod rpc;
mod struct_;

pub(crate) fn crate_name() -> syn::Ident {
    quote::format_ident!("atmo_core")
}

/// ATProto bindings generator.
#[derive(Default)]
pub struct Gen {
    ns_tree: NamespaceTree,
}

impl Gen {
    pub fn new() -> Gen {
        Gen::default()
    }

    pub fn add_lexicon(&mut self, source: &str) {
        let mut des = serde_json::Deserializer::from_str(source);
        let lex: Lexicon = serde_path_to_error::deserialize(&mut des).unwrap();

        self.ns_tree.add_lexicon(&lex);
    }

    pub fn generate(&self) -> TokenStream {
        let mod_tree = self.ns_tree.to_module_tree();
        mod_tree.to_token_stream()
    }
}

pub fn string_format_type(format: StringFormat) -> Type {
    match format {
        StringFormat::AtIdentifier => Type::AtIdentifier,
        StringFormat::AtUri => Type::AtUri,
        StringFormat::Cid => Type::CidString,
        StringFormat::Datetime => Type::DateTime,
        StringFormat::Did => Type::Did,
        StringFormat::Handle => Type::Handle,
        StringFormat::Language => Type::Language,
        StringFormat::Nsid => Type::Nsid,
        StringFormat::RecordKey => Type::RecordKey,
        StringFormat::Tid => Type::Tid,
        StringFormat::Uri => Type::Url,
    }
}

#[derive(Clone, Debug)]
pub enum Type {
    AtIdentifier,
    AtUri,
    Blob,
    Bool,
    Bytes,
    CidLink,
    CidString,
    DateTime,
    Did,
    Handle,
    Integer,
    Item(ItemPath),
    Language,
    Nsid,
    Nullable(Box<Type>),
    Option(Box<Type>),
    RecordKey,
    String,
    Tid,
    Unknown,
    Url,
    Vec(Box<Type>),
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = crate_name();

        let quoted = match self {
            Type::AtIdentifier => quote! { #crate_::AtIdentifier },
            Type::AtUri => quote! { #crate_::AtUri },
            Type::Blob => quote! { #crate_::Blob },
            Type::Bool => quote! { bool },
            Type::Bytes => quote! { bytes::Bytes },
            Type::CidLink => quote! { #crate_::CidLink },
            Type::CidString => quote! { #crate_::CidString },
            Type::DateTime => quote! { #crate_::DateTime },
            Type::Did => quote! { #crate_::Did },
            Type::Handle => quote! { #crate_::Handle },
            Type::Integer => quote! { i64 },
            Type::Item(i) => {
                i.to_tokens(tokens);
                return;
            }
            Type::Language => quote! { language_tags::LanguageTag },
            Type::Nsid => quote! { #crate_::Nsid },
            Type::Nullable(t) => quote! { #crate_::Nullable<#t> },
            Type::Option(t) => quote! { std::option::Option<#t> },
            Type::RecordKey => quote! { #crate_::RecordKey },
            Type::String => quote! { std::string::String },
            Type::Tid => quote! { #crate_::Tid },
            Type::Unknown => quote! { #crate_::Unknown },
            Type::Url => quote! { url::Url },
            Type::Vec(t) => quote! { std::vec::Vec<#t> },
        };

        quoted.to_tokens(tokens);
    }
}

impl From<ItemPath> for Type {
    #[inline]
    fn from(item: ItemPath) -> Self {
        Type::Item(item)
    }
}

impl From<&BuiltinTy> for Type {
    fn from(ty: &BuiltinTy) -> Self {
        match ty {
            BuiltinTy::Blob(_b) => Type::Blob,
            BuiltinTy::Boolean(_b) => Type::Bool,
            BuiltinTy::Bytes(_b) => Type::Bytes,
            BuiltinTy::CidLink => Type::CidLink,
            BuiltinTy::Integer(_i) => Type::Integer,
            BuiltinTy::String(s) => {
                let Some(f) = &s.format else {
                    return Type::String;
                };

                match f {
                    StringFormat::AtIdentifier => Type::AtIdentifier,
                    StringFormat::AtUri => Type::AtUri,
                    StringFormat::Cid => Type::CidString,
                    StringFormat::Datetime => Type::DateTime,
                    StringFormat::Did => Type::Did,
                    StringFormat::Handle => Type::Handle,
                    StringFormat::Language => Type::Language,
                    StringFormat::Nsid => Type::Nsid,
                    StringFormat::RecordKey => Type::RecordKey,
                    StringFormat::Tid => Type::Tid,
                    StringFormat::Uri => Type::Url,
                }
            }
            BuiltinTy::Unknown => Type::Unknown,
        }
    }
}
