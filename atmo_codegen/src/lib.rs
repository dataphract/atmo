use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use atmo::nsid::{self, FullReference, Nsid};
use atmo_lexicon::{FieldSchema, Lexicon, Object, Schema, StringFormat, Union};
use enum_::{StringEnumDef, StringEnumVariant, UnionEnumDef, UnionEnumVariant};
use heck::{ToPascalCase, ToSnakeCase};
use module::{Item, ItemPath, ModulePath, Output};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use struct_::{Field, StructDef};

mod enum_;
mod module;
mod struct_;
mod unions;

fn crate_name() -> syn::Ident {
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

    fn resolve_ref(&self, namespace: &Nsid, r: nsid::Reference) -> Referent<'_> {
        let (nsid, fragment) = match r {
            nsid::Reference::Full(full) => (full.clone_nsid(), full.clone_fragment()),
            nsid::Reference::Relative(fragment) => (namespace.clone(), Some(fragment)),
        };

        let name = fragment
            .as_ref()
            .and_then(|s| s.as_str().strip_prefix('#'))
            .unwrap_or("main");

        let schema = self.lexicons.get(&nsid).unwrap().defs.get(name).unwrap();

        Referent {
            path: ModulePath::from(nsid).item_path(name.to_pascal_case()),
            schema,
        }
    }

    fn resolve_string_type(
        &self,
        output: &mut Output,
        namespace: &Nsid,
        prop_name: &str,
        s: &atmo_lexicon::String,
    ) -> TokenStream {
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

            return self.emit_string_enum(output, namespace, prop_name, &s.known_values, true);
        }

        if let Some(enum_values) = &s.enum_values {
            assert!(s.const_value.is_none());

            return self.emit_string_enum(output, namespace, prop_name, enum_values, false);
        }

        quote! { std::string::String }
    }

    /// Emits an enum definition for a set of known string values.
    fn emit_string_enum(
        &self,
        output: &mut Output,
        namespace: &Nsid,
        prop_name: &str,
        known_values: &[String],
        is_open: bool,
    ) -> TokenStream {
        let type_name = prop_name.to_pascal_case();
        let type_ident = quote::format_ident!("{type_name}");

        let variants = known_values
            .iter()
            .map(|v| self.gen_string_enum_variant(namespace, v.borrow()))
            .collect();

        let mod_path = namespace.into();
        let module = output.get_or_create_mut(&mod_path);

        // Emit a definition for the string enum.
        module.add_item(
            type_name.clone(),
            Item::StringEnum(StringEnumDef {
                ident: type_ident,
                variants,
                is_open,
            }),
        );

        mod_path.item_path(type_name).into_token_stream()
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
        output: &mut Output,
        namespace: &Nsid,
        def_name: &str,
        prop_name: &str,
        schema: &Union,
    ) -> ItemPath {
        let variants = schema
            .refs
            .iter()
            .map(|s| {
                let r = nsid::Reference::from_str(s).unwrap();
                let referent = self.resolve_ref(namespace, r);

                let name = quote::format_ident!("{}", referent.path.name());
                let inner = referent.path.clone();

                UnionEnumVariant { name, inner }
            })
            .collect();

        let module_path = namespace.into();
        let module = output.get_or_create_mut(&module_path);

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

    fn emit_struct(&self, output: &mut Output, namespace: &Nsid, name: &str, object: &Object) {
        // Convert name to PascalCase.
        let type_name = name.to_pascal_case();
        let type_ident = quote::format_ident!("{type_name}");

        let required: HashSet<_> = object.required.iter().map(String::as_str).collect();
        let nullable: HashSet<_> = object.nullable.iter().map(String::as_str).collect();

        let mut fields = Vec::new();
        for (prop_name, prop_schema) in &object.properties {
            let field = self.emit_struct_field(
                output,
                namespace,
                name,
                prop_name,
                prop_schema,
                required.contains(prop_name.as_str()),
                nullable.contains(prop_name.as_str()),
            );
            fields.push(field);
        }

        output.get_or_create_mut(&namespace.into()).add_item(
            type_name,
            Item::Struct(StructDef {
                name: type_ident,
                fields,
            }),
        );
    }

    fn emit_struct_field(
        &self,
        output: &mut Output,
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
                    FieldSchema::CidLink => quote! { CidLink },
                    FieldSchema::Ref(r) => {
                        let nsid_ref = nsid::Reference::from_str(&r.ref_).unwrap();
                        let referent = self.resolve_ref(namespace, nsid_ref);
                        referent.path.into_token_stream()
                    }
                    FieldSchema::String(_) => quote! { String },
                    FieldSchema::Union(_) => quote! { Union },
                    FieldSchema::Unknown => quote! { Unknown },
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

            FieldSchema::CidLink => quote! { #crate_::cid::CidLink },

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

            FieldSchema::String(s) => self.resolve_string_type(output, namespace, prop_name, s),

            FieldSchema::Union(u) => {
                eprintln!("unions not properly implemented yet");

                self.emit_union(output, namespace, def_name, prop_name, u)
                    .into_token_stream()
            }

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

    pub fn generate(self) {
        let mut output = Output::default();

        for (nsid, lex) in self.lexicons.iter() {
            for (name, schema) in &lex.defs {
                // println!("{nsid}#{name}");
                let full = FullReference::from_str(&format!("{nsid}#{name}")).unwrap();

                let mut module_path = ModulePath::new();
                for segment in nsid.segments() {
                    module_path.push(segment.to_snake_case());
                }

                output.get_or_create_mut(&full.clone_nsid().into());

                let def_path = name.to_snake_case();

                match schema {
                    Schema::Array(a) => {
                        //println!("array def: {nsid}#{name} = {a:?}");
                        continue;
                    }
                    Schema::Object(o) => self.emit_struct(&mut output, nsid, name, o),
                    Schema::Record(r) => continue,
                    Schema::Procedure(_) | Schema::Query(_) => continue,
                    Schema::String(s) => {
                        let atmo_lexicon::String {
                            description,
                            format,
                            max_length,
                            min_length,
                            max_graphemes,
                            min_graphemes,
                            known_values,
                            enum_values,
                            default,
                            const_value,
                        } = s;

                        assert!(format.is_none());
                        assert!(const_value.is_none());

                        continue;
                    }
                    Schema::Subscription(_) => continue,
                    Schema::Token(_) => continue,
                    s => panic!("unhandled top-level definition: {s:?}"),
                };
            }
        }

        println!("\n{}", output.to_token_stream());
    }
}

pub fn string_format_type(format: StringFormat) -> TokenStream {
    let crate_ = crate_name();

    match format {
        StringFormat::AtIdentifier => quote! { #crate_::at_uri::AtIdentifier },
        StringFormat::AtUri => quote! { #crate_::at_uri::AtUri },
        StringFormat::Cid => quote! { #crate_::cid::CidString },
        StringFormat::Datetime => quote! { #crate_::datetime::DateTimeString },
        StringFormat::Did => quote! { #crate_::did::Did },
        StringFormat::Handle => quote! { #crate_::handle::Handle },
        StringFormat::Language => {
            eprintln!("string format `{format:?}` not handled yet");
            quote! { std::string::String }
        }
        StringFormat::Nsid => quote! { #crate_::nsid::Nsid },
        StringFormat::RecordKey => quote! { #crate_::rkey::RecordKey },
        StringFormat::Tid => quote! { #crate_::tid::Tid },
        StringFormat::Uri => quote! { url::Url },
    }
}

pub struct Referent<'a> {
    path: ItemPath,
    schema: &'a Schema,
}
