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
//!
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
//!     - If the object appears in

use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use atmo_core::nsid::{self, FullReference, Nsid, Reference};
use atmo_lexicon::{
    FieldSchema, Input, IoSchema, Lexicon, Object, Output, Schema, StringFormat, Union,
};
use enum_::{StringEnumDef, StringEnumVariant, UnionEnumDef, UnionEnumVariant};
use heck::{ToPascalCase, ToSnakeCase};
use module::{Generated, Item, ItemPath, ModulePath};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use rpc::{RpcDef, RpcType};
use struct_::{Field, StructDef};

mod enum_;
mod module;
mod rpc;
mod struct_;
mod unions;

pub(crate) fn crate_name() -> syn::Ident {
    quote::format_ident!("atmo_core")
}

/// ATProto bindings generator.
#[derive(Default)]
pub struct Gen {
    // Input Lexicon schemae.
    lexicons: BTreeMap<Nsid, Lexicon>,
}

impl Gen {
    pub fn new() -> Gen {
        Gen::default()
    }

    pub fn add_lexicon(&mut self, source: &str) {
        let mut des = serde_json::Deserializer::from_str(source);
        let lex: Lexicon = serde_path_to_error::deserialize(&mut des).unwrap();

        let nsid = Nsid::from_str(lex.id.as_str()).unwrap();

        // Insert the new lexicon.
        self.lexicons.insert(nsid.clone(), lex);
    }

    pub fn generate(self) {
        let mut generated = Generated::default();

        for (nsid, lex) in self.lexicons.iter() {
            for (name, schema) in &lex.defs {
                let full = FullReference::from_str(&format!("{nsid}#{name}")).unwrap();

                let mut module_path = ModulePath::new();
                for segment in nsid.segments() {
                    module_path.push(segment.to_snake_case());
                }

                generated.get_or_create_mut(&full.clone_nsid().into());

                let mut scoped = ScopedGen {
                    generated: &mut generated,
                    scope: nsid.clone(),
                    gen: &self,
                };

                match schema {
                    // Array defs are only looked up for use as struct fields.
                    Schema::Array(a) => {
                        scoped.emit_struct_field(&full, name, &a.items, true, false);
                    }

                    Schema::Object(o) => {
                        let full = FullReference::from_str(&format!("{nsid}#{name}")).unwrap();
                        scoped.emit_struct(&full, o);
                    }

                    Schema::Record(r) => {
                        let full = FullReference::from_str(nsid.as_str()).unwrap();
                        scoped.emit_struct(&full, &r.record);
                    }

                    Schema::Procedure(p) => {
                        scoped.emit_rpc(
                            RpcType::Procedure,
                            p.parameters.as_ref(),
                            p.input.as_ref(),
                            p.output.as_ref(),
                        );
                    }

                    Schema::Query(q) => {
                        scoped.emit_rpc(
                            RpcType::Query,
                            q.parameters.as_ref(),
                            None,
                            q.output.as_ref(),
                        );
                    }

                    Schema::String(s) => {
                        // TODO(dp) this is hacky, make a dedicated method for strings in defs
                        scoped.resolve_string_type(name, s);
                    }

                    Schema::Subscription(_) => continue,

                    Schema::Token(_) => {
                        // Don't need to generate a def, as these are only referenced by other definitions.
                        //
                        // TODO(dp): Might still be nice to generate constants, though?
                        continue;
                    }

                    s => panic!("unhandled top-level definition: {s:?}"),
                };
            }
        }

        println!("\n{}", generated.to_token_stream());
    }
}

pub struct ScopedGen<'gen, 'lex> {
    generated: &'gen mut Generated,
    scope: Nsid,
    gen: &'lex Gen,
}

impl<'gen, 'lex> ScopedGen<'gen, 'lex> {
    fn resolve_ref(&self, r: nsid::Reference) -> Referent<'lex> {
        let (nsid, fragment) = match r {
            nsid::Reference::Full(full) => (full.clone_nsid(), full.clone_fragment()),
            nsid::Reference::Relative(fragment) => (self.scope.clone(), Some(fragment)),
        };

        let fragment_s = fragment.as_ref().map(|f| f.as_str()).unwrap_or("#main");

        let full = FullReference::from_str(&format!("{nsid}{fragment_s}")).unwrap();
        let item_path = struct_path(&full);

        let name = fragment
            .as_ref()
            .and_then(|s| s.as_str().strip_prefix('#'))
            .unwrap_or("main");

        let schema = self
            .gen
            .lexicons
            .get(&nsid)
            .unwrap_or_else(|| panic!("error loading Lexicon for {nsid}"))
            .defs
            .get(name)
            .unwrap_or_else(|| panic!("no def for {nsid}#{name}"));

        Referent {
            full,
            path: item_path,
            schema,
        }
    }

    fn resolve_string_type(&mut self, prop_name: &str, s: &atmo_lexicon::String) -> Type {
        // Check if the string is a known format.
        if let Some(format) = s.format {
            assert!(s.known_values.is_empty());
            assert!(s.enum_values.is_none());
            assert!(s.default.is_none());
            assert!(s.const_value.is_none());

            return string_format_type(format);
        }

        // If the string has known values, emit an open enum.
        if !s.known_values.is_empty() {
            assert!(s.enum_values.is_none());
            assert!(s.const_value.is_none());

            let item = self.emit_string_enum(prop_name, &s.known_values, true);
            return Type::Item(item);
        }

        // If the string is an enum, emit a closed enum.
        if let Some(enum_values) = &s.enum_values {
            assert!(s.const_value.is_none());

            let item = self.emit_string_enum(prop_name, enum_values, false);
            return Type::Item(item);
        }

        Type::String
    }

    /// Emits an enum definition for a set of known string values.
    fn emit_string_enum(
        &mut self,
        prop_name: &str,
        known_values: &[String],
        is_open: bool,
    ) -> ItemPath {
        let type_name = prop_name.to_pascal_case();
        let type_ident = quote::format_ident!("{type_name}");

        let variants = known_values
            .iter()
            .map(|v| self.gen_string_enum_variant(v.borrow()))
            .collect();

        let mod_path = self.scope.clone().into();
        let module = self.generated.get_or_create_mut(&mod_path);

        // Emit a definition for the string enum.
        module
            .add_item(
                type_name.clone(),
                Item::StringEnum(StringEnumDef {
                    ident: type_ident,
                    variants,
                    is_open,
                }),
            )
            .unwrap();

        mod_path.item_path(type_name)
    }

    /// Generates a single variant for an enum of known string values.
    fn gen_string_enum_variant(&mut self, value: &str) -> StringEnumVariant {
        let ident = quote::format_ident!("{}", value.to_pascal_case());

        // If the value isn't a reference, create a variant of the same name and return.
        if !value.contains('#') {
            return StringEnumVariant {
                doc: None,
                string_value: value.into(),
                ident,
            };
        }

        // If the value is a reference, resolve it.
        let referent = self.resolve_ref(nsid::Reference::from_str(value).unwrap());

        match referent.schema {
            Schema::Token(t) => StringEnumVariant {
                doc: t.description.clone(),
                string_value: value.into(),
                ident: quote::format_ident!("{}", referent.path.name()),
            },
            _ => panic!("unsupported string ref type: {:?}", referent.schema),
        }
    }

    fn emit_union(&mut self, full: &FullReference, prop_name: &str, schema: &Union) -> ItemPath {
        let variants = schema
            .refs
            .iter()
            .map(|s| {
                let r = nsid::Reference::from_str(s).unwrap();
                let referent = self.resolve_ref(r);
                let path = referent.path.clone();

                // TODO(dp): this is fragile, needs to actually do a full walk of references. also
                // boxes too eagerly -- there might be a Vec indirection, so need to keep track of
                // that and avoid boxing if possible.
                let needs_boxed = &referent.full == full;

                UnionEnumVariant {
                    nsid: referent.full,
                    path,
                    needs_boxed,
                }
            })
            .collect();

        let module_path = (&self.scope).into();
        let module = self.generated.get_or_create_mut(&module_path);

        let mut type_name = prop_name.to_pascal_case();

        if module.item_exists(&type_name) {
            // Glue the def name on and hope that fixes the problem.
            let def_name = full.fragment_name().unwrap();
            type_name = format!("{def_name}_{prop_name}").to_pascal_case();
        }

        let type_ident = quote::format_ident!("{type_name}");

        module
            .add_item(
                type_name.clone(),
                Item::UnionEnum(UnionEnumDef {
                    doc: schema.description.clone(),
                    ident: type_ident,
                    variants,
                    is_open: !schema.closed,
                }),
            )
            .unwrap();

        module_path.item_path(type_name)
    }

    fn emit_struct(&mut self, r: &FullReference, object: &Object) -> ItemPath {
        let item_path = struct_path(&r);
        let type_ident = quote::format_ident!("{}", item_path.name());

        let required: HashSet<_> = object.required.iter().map(String::as_str).collect();
        let nullable: HashSet<_> = object.nullable.iter().map(String::as_str).collect();

        let mut fields = Vec::new();
        for (prop_name, prop_schema) in &object.properties {
            let field = self.emit_struct_field(
                r,
                prop_name,
                prop_schema,
                required.contains(prop_name.as_str()),
                nullable.contains(prop_name.as_str()),
            );
            fields.push(field);
        }

        self.generated
            .get_or_create_mut(item_path.module_path())
            .add_item(
                item_path.name().into(),
                Item::Struct(StructDef {
                    name: type_ident,
                    fields,
                }),
            )
            .unwrap();

        item_path
    }

    /// Returns a descriptor for a struct field.
    ///
    /// Emits definitions for any undefined constituent types of the field type.
    fn emit_struct_field(
        &mut self,
        full: &FullReference,
        prop_name: &str,
        schema: &FieldSchema,
        required: bool,
        nullable: bool,
    ) -> Field {
        let mut desc = None;

        // Rename fields if they would collide with Rust keywords.
        let field_name = {
            let name = prop_name.to_snake_case();

            match name.as_str() {
                "ref" => "ref_".into(),
                "type" => "ty".into(),
                _ => name,
            }
        };

        let field_ident = quote::format_ident!("{field_name}");

        let inner_ty: Type = match schema {
            FieldSchema::Array(a) => {
                let elem_ty = match &*a.items {
                    FieldSchema::CidLink => Type::CidString,
                    FieldSchema::Ref(r) => {
                        let nsid_ref = nsid::Reference::from_str(&r.ref_).unwrap();
                        let referent = self.resolve_ref(nsid_ref);
                        referent.path.into()
                    }
                    FieldSchema::String(_) => Type::String,
                    FieldSchema::Union(u) => self.emit_union(full, prop_name, u).into(),
                    FieldSchema::Unknown => Type::Unknown,
                    x => panic!("unhandled array element type: {x:?}"),
                };

                Type::Vec(elem_ty.into())
            }

            FieldSchema::Blob(b) => {
                desc = b.description.clone();
                eprintln!("blob MIME type and size are not enforced");
                Type::Blob
            }

            FieldSchema::Boolean(b) => {
                desc = b.description.clone();
                Type::Bool
            }

            FieldSchema::Bytes(b) => {
                desc = b.description.clone();
                Type::Bytes
            }

            FieldSchema::CidLink => Type::CidLink,

            FieldSchema::Integer(i) => {
                // TODO: deranged
                if i.minimum.is_some() || i.maximum.is_some() {
                    eprintln!("ranged integer support not implemented yet sorry lol");
                }

                Type::Integer
            }

            FieldSchema::Ref(r) => {
                let nsid_ref = nsid::Reference::from_str(&r.ref_).unwrap();
                let referent = self.resolve_ref(nsid_ref);

                // TODO(dp): This is a hack. If the array element type has a name (e.g. it's a union
                // or struct) then the ref will resolve to the element type instead of the array, so
                // we need to manually resolve it to an array.
                match referent.schema {
                    Schema::Array(_) => Type::Vec(Box::new(referent.path.into())),
                    _ => referent.path.into(),
                }
            }

            FieldSchema::String(s) => self.resolve_string_type(prop_name, s),
            FieldSchema::Union(u) => self.emit_union(full, prop_name, u).into(),
            FieldSchema::Unknown => Type::Unknown,
        };

        Field {
            doc: desc,
            name: field_ident,
            rename: prop_name.into(),
            optional: !required,
            nullable,
            inner_ty,
        }
    }

    fn emit_rpc_io(&mut self, prop_name: &str, schema: &IoSchema) -> ItemPath {
        let nsid = self.scope.clone();
        let full = FullReference::from_str(&format!("{nsid}#{prop_name}")).unwrap();

        match schema {
            IoSchema::Object(o) => self.emit_struct(&full, o),

            IoSchema::Ref(r) => {
                let reference = Reference::from_str(&r.ref_).unwrap();
                let referent = self.resolve_ref(reference);
                referent.path.clone()
            }

            IoSchema::Union(u) => self.emit_union(&full, prop_name, u),
        }
    }

    /// Emits an RPC (query or procedure) and its input and output types.
    ///
    /// For an RPC with NSID `foo.bar.baz.qux`, the RPC type is emitted at `foo::bar::baz::Qux`, and
    /// its I/O types are emitted at `foo::bar::baz::qux::{Params, Input, Output}`.
    fn emit_rpc(
        &mut self,
        ty: RpcType,
        params: Option<&Object>,
        input_schema: Option<&Input>,
        output_schema: Option<&Output>,
    ) {
        let nsid = self.scope.clone();
        let params_full = FullReference::from_str(&format!("{nsid}#params")).unwrap();

        let params = params.map(|params| self.emit_struct(&params_full, params));

        let input = input_schema
            .and_then(|input| input.schema.as_ref())
            .map(|io| self.emit_rpc_io("input", io));

        let (output, output_encoding) = match output_schema {
            Some(o) => {
                let output = o.schema.as_ref().map(|io| self.emit_rpc_io("output", io));
                (output, o.encoding.clone())
            }
            None => (None, "*/*".to_owned()),
        };

        let path = ModulePath::from(&self.scope);
        let rpc_name = path.name().to_pascal_case();
        let ident = quote::format_ident!("{rpc_name}");
        let parent_path = path.parent().unwrap();

        self.generated
            .get_or_create_mut(&parent_path)
            .add_item(
                rpc_name,
                Item::Rpc(RpcDef {
                    ident,
                    ty,
                    nsid: self.scope.clone(),
                    params,
                    input,
                    output,
                    output_encoding,
                }),
            )
            .unwrap();
    }
}

pub fn string_format_type(format: StringFormat) -> Type {
    match format {
        StringFormat::AtIdentifier => Type::AtIdentifier,
        StringFormat::AtUri => Type::AtUri,
        StringFormat::Cid => Type::CidString,
        StringFormat::Datetime => Type::DatetimeString,
        StringFormat::Did => Type::Did,
        StringFormat::Handle => Type::Handle,
        StringFormat::Language => Type::Language,
        StringFormat::Nsid => Type::Nsid,
        StringFormat::RecordKey => Type::RecordKey,
        StringFormat::Tid => Type::Tid,
        StringFormat::Uri => Type::Url,
    }
}

#[derive(Debug)]
pub struct Referent<'a> {
    full: FullReference,
    path: ItemPath,
    schema: &'a Schema,
}

#[derive(Debug)]
pub enum Type {
    AtIdentifier,
    AtUri,
    Blob,
    Bool,
    Bytes,
    CidLink,
    CidString,
    DatetimeString,
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
            Type::DatetimeString => quote! { #crate_::DateTimeString },
            Type::Did => quote! { #crate_::Did },
            Type::Handle => quote! { #crate_::Handle },
            Type::Integer => quote! { i64 },
            Type::Item(i) => {
                i.to_tokens(tokens);
                return;
            }
            Type::Language => {
                eprintln!("language tags not implemented yet");
                quote! { std::string::String }
            }
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

fn struct_path(r: &FullReference) -> ItemPath {
    let mod_path = ModulePath::from(r.clone_nsid());

    match r.fragment_name().filter(|&n| n != "main") {
        Some(n) => mod_path.item_path(n.to_pascal_case()),

        None => {
            let name = mod_path.name().to_pascal_case();
            let parent_path = mod_path.parent().unwrap();
            parent_path.item_path(name)
        }
    }
}
