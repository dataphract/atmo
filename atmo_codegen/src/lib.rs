use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use atmo::nsid::{self, FullReference, Nsid, Reference};
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

const APPLICATION_JSON: &str = "application/json";

pub(crate) fn crate_name() -> syn::Ident {
    quote::format_ident!("atmo")
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

    /// Resolves an `nsid::Reference` to a `Referent`.
    fn resolve_ref(&self, namespace: &Nsid, r: nsid::Reference) -> Referent<'_> {
        let (nsid, fragment) = match r {
            nsid::Reference::Full(full) => (full.clone_nsid(), full.clone_fragment()),
            nsid::Reference::Relative(fragment) => (namespace.clone(), Some(fragment)),
        };

        let name = fragment
            .as_ref()
            .and_then(|s| s.as_str().strip_prefix('#'))
            .unwrap_or("main");

        let schema = self
            .lexicons
            .get(&nsid)
            .unwrap_or_else(|| panic!("error loading Lexicon for {nsid}"))
            .defs
            .get(name)
            .unwrap();

        Referent {
            full: FullReference::from_str(&format!("{namespace}#{name}")).unwrap(),
            path: ModulePath::from(nsid).item_path(name.to_pascal_case()),
            schema,
        }
    }

    fn resolve_string_type(
        &self,
        generated: &mut Generated,
        namespace: &Nsid,
        prop_name: &str,
        s: &atmo_lexicon::String,
    ) -> Type {
        if let Some(format) = s.format {
            assert!(s.known_values.is_empty());
            assert!(s.enum_values.is_none());
            assert!(s.default.is_none());
            assert!(s.const_value.is_none());

            return string_format_type(format);
        }

        if !s.known_values.is_empty() {
            assert!(s.enum_values.is_none());
            assert!(s.const_value.is_none());

            let item =
                self.emit_string_enum(generated, namespace, prop_name, &s.known_values, true);
            return Type::Item(item);
        }

        if let Some(enum_values) = &s.enum_values {
            assert!(s.const_value.is_none());

            let item = self.emit_string_enum(generated, namespace, prop_name, enum_values, false);
            return Type::Item(item);
        }

        Type::String
    }

    /// Emits an enum definition for a set of known string values.
    fn emit_string_enum(
        &self,
        generated: &mut Generated,
        namespace: &Nsid,
        prop_name: &str,
        known_values: &[String],
        is_open: bool,
    ) -> ItemPath {
        let type_name = prop_name.to_pascal_case();
        let type_ident = quote::format_ident!("{type_name}");

        let variants = known_values
            .iter()
            .map(|v| self.gen_string_enum_variant(namespace, v.borrow()))
            .collect();

        let mod_path = namespace.into();
        let module = generated.get_or_create_mut(&mod_path);

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
    fn gen_string_enum_variant(&self, namespace: &Nsid, value: &str) -> StringEnumVariant {
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
        let referent = self.resolve_ref(namespace, nsid::Reference::from_str(value).unwrap());

        match referent.schema {
            Schema::Token(t) => StringEnumVariant {
                doc: t.description.clone(),
                string_value: value.into(),
                ident: quote::format_ident!("{}", referent.path.name()),
            },
            _ => panic!("unsupported string ref type: {:?}", referent.schema),
        }
    }

    fn emit_union(
        &self,
        generated: &mut Generated,
        namespace: &Nsid,
        def_name: &str,
        prop_name: &str,
        schema: &Union,
    ) -> ItemPath {
        let full = FullReference::from_str(&format!("{namespace}#{def_name}")).unwrap();

        let variants = schema
            .refs
            .iter()
            .map(|s| {
                let r = nsid::Reference::from_str(s).unwrap();
                let referent = self.resolve_ref(namespace, r);
                let path = referent.path.clone();

                // TODO(dp): this is fragile, needs to actually do a full walk of references. also
                // boxes too eagerly -- there might be a Vec indirection, so need to keep track of
                // that and avoid boxing if possible.
                let needs_boxed = referent.full == full;
                eprintln!(
                    "{def_name}.{prop_name}: {} == {} : {needs_boxed}",
                    referent.full, full
                );

                UnionEnumVariant { path, needs_boxed }
            })
            .collect();

        let module_path = namespace.into();
        let module = generated.get_or_create_mut(&module_path);

        let mut type_name = prop_name.to_pascal_case();

        if module.item_exists(&type_name) {
            // Glue the def name on and hope that fixes the problem.
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

    fn emit_struct(
        &self,
        generated: &mut Generated,
        namespace: &Nsid,
        name: &str,
        object: &Object,
    ) -> ItemPath {
        // Convert name to PascalCase.
        let type_name = name.to_pascal_case();
        let type_ident = quote::format_ident!("{type_name}");

        let required: HashSet<_> = object.required.iter().map(String::as_str).collect();
        let nullable: HashSet<_> = object.nullable.iter().map(String::as_str).collect();

        let mut fields = Vec::new();
        for (prop_name, prop_schema) in &object.properties {
            let field = self.emit_struct_field(
                generated,
                namespace,
                name,
                prop_name,
                prop_schema,
                required.contains(prop_name.as_str()),
                nullable.contains(prop_name.as_str()),
            );
            fields.push(field);
        }

        let item_path = ModulePath::from(namespace).item_path(type_name.clone());

        generated
            .get_or_create_mut(&namespace.into())
            .add_item(
                type_name,
                Item::Struct(StructDef {
                    name: type_ident,
                    fields,
                }),
            )
            .unwrap();

        item_path
    }

    fn emit_struct_field(
        &self,
        generated: &mut Generated,
        namespace: &Nsid,
        def_name: &str,
        prop_name: &str,
        schema: &FieldSchema,
        required: bool,
        nullable: bool,
    ) -> Field {
        let crate_ = crate::crate_name();

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

        let inner_ty: TokenStream = match schema {
            FieldSchema::Array(a) => {
                let elem_ty = match &*a.items {
                    FieldSchema::CidLink => quote! { atmo::CidLink },
                    FieldSchema::Ref(r) => {
                        let nsid_ref = nsid::Reference::from_str(&r.ref_).unwrap();
                        let referent = self.resolve_ref(namespace, nsid_ref);
                        referent.path.into_token_stream()
                    }
                    FieldSchema::String(_) => quote! { std::string::String },
                    FieldSchema::Union(u) => self
                        .emit_union(generated, namespace, def_name, prop_name, u)
                        .into_token_stream(),
                    FieldSchema::Unknown => quote! { atmo::Unknown },
                    x => panic!("unhandled array element type: {x:?}"),
                };

                quote! { Vec<#elem_ty> }
            }

            FieldSchema::Blob(b) => {
                desc = b.description.clone();
                eprintln!("blob MIME type and size are not enforced");
                quote! { atmo::Blob }
            }

            FieldSchema::Boolean(b) => {
                desc = b.description.clone();
                quote::format_ident!("bool").to_token_stream()
            }

            FieldSchema::Bytes(b) => {
                desc = b.description.clone();
                quote! { Vec<u8> }
            }

            FieldSchema::CidLink => quote! { #crate_::CidLink },

            FieldSchema::Integer(i) => {
                // TODO: deranged
                if i.minimum.is_some() || i.maximum.is_some() {
                    eprintln!("ranged integer support not implemented yet sorry lol");
                }

                quote! { i64 }
            }

            FieldSchema::Ref(r) => {
                let nsid_ref = nsid::Reference::from_str(&r.ref_).unwrap();
                let referent = self.resolve_ref(namespace, nsid_ref);
                referent.path.into_token_stream()
            }

            FieldSchema::String(s) => self
                .resolve_string_type(generated, namespace, prop_name, s)
                .into_token_stream(),

            FieldSchema::Union(u) => self
                .emit_union(generated, namespace, def_name, prop_name, u)
                .into_token_stream(),

            FieldSchema::Unknown => {
                eprintln!("unknown fields not properly implemented yet");
                quote! { () }
            }
        };

        Field {
            doc: desc,
            name: field_ident,
            optional: !required,
            nullable,
            inner_ty,
        }
    }

    fn emit_rpc_io(
        &self,
        generated: &mut Generated,
        namespace: &Nsid,
        def_name: &str,
        prop_name: &str,
        schema: &IoSchema,
    ) -> ItemPath {
        match schema {
            IoSchema::Object(o) => self.emit_struct(generated, namespace, prop_name, o),

            IoSchema::Ref(r) => {
                let reference = Reference::from_str(&r.ref_).unwrap();
                let referent = self.resolve_ref(namespace, reference);
                referent.path.clone()
            }

            IoSchema::Union(u) => self.emit_union(generated, namespace, def_name, prop_name, u),
        }
    }

    /// Emits an RPC (query or procedure) and its input and output types.
    ///
    /// For an RPC with NSID `foo.bar.baz.qux`, the RPC type is emitted at `foo::bar::baz::Qux`, and
    /// its I/O types are emitted at `foo::bar::baz::qux::{Params, Input, Output}`.
    fn emit_rpc(
        &self,
        generated: &mut Generated,
        namespace: &Nsid,
        def_name: &str,
        ty: RpcType,
        params: Option<&Object>,
        input_schema: Option<&Input>,
        output_schema: Option<&Output>,
    ) {
        let params = params.map(|params| self.emit_struct(generated, namespace, "params", params));

        let input = input_schema
            .and_then(|input| input.schema.as_ref())
            .map(|io| self.emit_rpc_io(generated, namespace, def_name, "input", io));

        let (output, output_encoding) = match output_schema {
            Some(o) => {
                let output = o
                    .schema
                    .as_ref()
                    .map(|io| self.emit_rpc_io(generated, namespace, def_name, "output", io));
                (output, o.encoding.clone())
            }
            None => (None, "*/*".to_owned()),
        };

        let path = ModulePath::from(namespace);
        let rpc_name = path.name().to_pascal_case();
        let ident = quote::format_ident!("{rpc_name}");
        let parent_path = path.parent().unwrap();

        generated.get_or_create_mut(&parent_path).add_item(
            rpc_name,
            Item::Rpc(RpcDef {
                ident,
                ty,
                nsid: namespace.clone(),
                params,
                input,
                output,
                output_encoding,
            }),
        );
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

                let def_path = name.to_snake_case();

                match schema {
                    Schema::Array(a) => {
                        //println!("array def: {nsid}#{name} = {a:?}");
                        continue;
                    }

                    Schema::Object(o) => {
                        self.emit_struct(&mut generated, nsid, name, o);
                    }

                    Schema::Record(r) => continue,

                    Schema::Procedure(p) => {
                        self.emit_rpc(
                            &mut generated,
                            nsid,
                            name,
                            RpcType::Procedure,
                            p.parameters.as_ref(),
                            p.input.as_ref(),
                            p.output.as_ref(),
                        );
                    }

                    Schema::Query(q) => {
                        self.emit_rpc(
                            &mut generated,
                            nsid,
                            name,
                            RpcType::Procedure,
                            q.parameters.as_ref(),
                            None,
                            q.output.as_ref(),
                        );
                    }

                    Schema::String(s) => {
                        // TODO(dp) this is hacky, make a dedicated method for strings in defs
                        self.resolve_string_type(&mut generated, nsid, name, s);
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

pub fn string_format_type(format: StringFormat) -> Type {
    match format {
        StringFormat::AtIdentifier => Type::AtIdentifier,
        StringFormat::AtUri => Type::AtUri,
        StringFormat::Cid => Type::Cid,
        StringFormat::Datetime => Type::Datetime,
        StringFormat::Did => Type::Did,
        StringFormat::Handle => Type::Handle,
        StringFormat::Language => Type::Language,
        StringFormat::Nsid => Type::Nsid,
        StringFormat::RecordKey => Type::RecordKey,
        StringFormat::Tid => Type::Tid,
        StringFormat::Uri => Type::Url,
    }
}

pub struct Referent<'a> {
    full: FullReference,
    path: ItemPath,
    schema: &'a Schema,
}

pub enum Type {
    AtIdentifier,
    AtUri,
    Blob,
    Cid,
    Datetime,
    Did,
    Handle,
    Item(ItemPath),
    Language,
    Nsid,
    Nullable(Box<Type>),
    Option(Box<Type>),
    RecordKey,
    String,
    Tid,
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
            Type::Cid => quote! { #crate_::Cid },
            Type::Datetime => quote! { #crate_::DateTimeString },
            Type::Did => quote! { #crate_::Did },
            Type::Handle => quote! { #crate_::Handle },
            Type::Item(i) => {
                i.to_tokens(tokens);
                return;
            }
            Type::Language => quote! { #crate_::Language },
            Type::Nsid => quote! { #crate_::Nsid },
            Type::Nullable(t) => quote! { #crate_::Nullable<#t> },
            Type::Option(t) => quote! { std::option::Option<#t> },
            Type::RecordKey => quote! { #crate_::RecordKey },
            Type::String => quote! { std::string::String },
            Type::Tid => quote! { #crate_::Tid },
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
