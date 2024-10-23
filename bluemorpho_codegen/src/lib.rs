use std::collections::{BTreeMap, HashSet};

use bluemorpho::{
    lexicon::{Object, Schema, StringFormat},
    nsid::Nsid,
    Lexicon,
};
use heck::{ToPascalCase, ToSnakeCase};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

/// ATProto bindings generator.
#[derive(Default)]
pub struct Gen {
    lexicons: BTreeMap<Nsid, Lexicon>,
    namespaces: BTreeMap<String, HashSet<Item>>,
}

impl Gen {
    pub fn new() -> Gen {
        Gen::default()
    }

    pub fn add_lexicon(&mut self, source: &str) {
        let mut des = serde_json::Deserializer::from_str(source);
        let lex: Lexicon = serde_path_to_error::deserialize(&mut des).unwrap();

        let nsid = Nsid::new(lex.id.clone()).unwrap();

        let (parent, _) = nsid.as_str().rsplit_once('.').unwrap();

        // Insert the new lexicon.
        self.lexicons.insert(nsid.clone(), lex);

        // Insert the immediate parent namespace.
        self.namespaces
            .entry(parent.into())
            .or_default()
            .insert(Item::Nsid(nsid.clone()));

        // Insert all ancestor namespaces.
        let mut child = parent;
        while let Some((parent, _)) = child.rsplit_once('.') {
            self.namespaces
                .entry(parent.into())
                .or_default()
                .insert(Item::Ns(child.into()));
            child = parent;
        }
    }

    pub fn generate(self) {
        for (name, items) in &self.namespaces {
            eprintln!("{name}: {} items", items.len());
        }

        for (nsid, lex) in &self.lexicons {
            for (name, schema) in &lex.defs {
                let fqn = if name == "main" {
                    format!("{nsid}")
                } else {
                    format!("{nsid}#{name}")
                };

                let mut module_path = String::new();
                for segment in nsid.segments() {
                    module_path.push_str("::");
                    module_path.push_str(&segment.to_snake_case());
                }

                let def_path = name.to_snake_case();

                let params = match schema {
                    Schema::Procedure(p) => p.parameters.as_ref(),
                    Schema::Query(q) => q.parameters.as_ref(),
                    _ => None,
                };

                let input = match schema {
                    Schema::Procedure(p) => p.input.as_ref(),
                    _ => None,
                };

                let output = match schema {
                    Schema::Procedure(p) => p.output.as_ref(),
                    Schema::Query(q) => q.output.as_ref(),
                    _ => None,
                };

                match schema {
                    Schema::Boolean(b) => {
                        panic!()
                    }
                    Schema::Object(o) => {
                        let typename = quote::format_ident!("{}", name.to_pascal_case());

                        let mut props = TokenStream::new();
                        for (prop_name, prop_schema) in &o.properties {
                            props.extend(emit_struct_field(prop_name, prop_schema));
                        }

                        let tokens = quote! {
                            pub struct #typename {
                                #props
                            }
                        };

                        println!("{tokens}");
                    }

                    Schema::Record(r) => {
                        let tokens = emit_struct("Record", &r.record);

                        println!("{tokens}");
                    }

                    //Schema::Procedure(_) | Schema::Query(_) => {
                    //println!(
                    //"fn {module_path}::call({}) {}",
                    //input.map(|_| "Input").unwrap_or(""),
                    //output.map(|_| "-> Output").unwrap_or("")
                    //);
                    //}
                    _ => (),
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Item {
    Ns(String),
    Nsid(Nsid),
}

pub fn emit_struct(name: &str, object: &Object) -> TokenStream {
    let typename = quote::format_ident!("{}", name.to_pascal_case());

    let mut props = TokenStream::new();
    for (prop_name, prop_schema) in &object.properties {
        props.extend(emit_struct_field(prop_name, prop_schema));
    }

    quote! {
        pub struct #typename {
            #props
        }
    }
}

pub fn emit_struct_field(prop_name: &str, schema: &Schema) -> TokenStream {
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

    let field_ident = syn::Ident::new(&field_name, Span::call_site());

    let ty: TokenStream = match schema {
        Schema::Array(a) => {
            let elem_ty = match &*a.items {
                Schema::CidLink => quote! { CidLink },
                Schema::Ref(_) => quote! { () },
                Schema::String(_) => quote! { String },
                Schema::Union(_) => quote! { Union },
                Schema::Unknown => quote! { Unknown },
                x => panic!("unhandled array element type: {x:?}"),
            };

            quote! { Vec<#elem_ty> }
        }

        Schema::Blob(b) => quote::format_ident!("Blob").to_token_stream(),

        Schema::Boolean(b) => {
            desc = b.description.clone();
            quote::format_ident!("bool").to_token_stream()
        }

        Schema::Bytes(b) => {
            desc = b.description.clone();
            quote! { Vec<u8> }
        }

        Schema::CidLink => {
            eprintln!("cid-link not implemented yet");
            quote! { () }
        }

        Schema::Integer(i) => {
            // TODO: deranged
            if i.minimum.is_some() || i.maximum.is_some() {
                eprintln!("ranged integer support not implemented yet sorry lol");
            }

            quote! { i64 }
        }

        Schema::Null => panic!("struct field can't have null type"),
        Schema::Object(_) => panic!("struct field can't define a new object type"),
        Schema::Params(_) => panic!("struct field can't define a new params type"),
        Schema::Procedure(_) => panic!("struct field can't define a procedure"),
        Schema::Query(_) => panic!("struct field can't define a query"),
        Schema::Record(_) => panic!("struct field can't define a record type"),

        Schema::Ref(_) => {
            eprintln!("Ref resolution not implemented yet");
            quote! { () }
        }

        Schema::String(s) => emit_string_type(s),

        Schema::Subscription(_) => panic!("struct field can't define a subscription type"),
        Schema::Token => panic!("struct field can't define a token"),
        Schema::Union(u) => {
            eprintln!("unions not properly implemented yet");

            eprintln!("union refs: {:?}", &u.refs);

            quote! { () }
        }

        Schema::Unknown => {
            eprintln!("unknown fields not properly implemented yet");
            quote! { () }
        }
    };

    let doc_attr = desc.map(|d| {
        quote! {
            #[doc = #d]
        }
    });

    quote! {
        #doc_attr
        #field_ident: #ty,
    }
}

fn emit_string_type(s: &bluemorpho::lexicon::String) -> TokenStream {
    let Some(format) = &s.format else {
        return quote! { std::string::String };
    };

    match format {
        StringFormat::Datetime => quote! { bluemorpho::datetime::DateTimeString },
        StringFormat::Did => quote! { bluemorpho::did::Did },
        StringFormat::Handle => quote! { bluemorpho::handle::Handle },
        StringFormat::Language => quote! { std::string::String },
        StringFormat::Nsid => quote! { bluemorpho::nsid::Nsid },
        StringFormat::RecordKey => quote! { bluemorpho::rkey::RecordKey },
        StringFormat::Tid => quote! { bluemorpho::tid::Tid },
        _ => {
            eprintln!("string format `{format:?}` not handled yet");
            quote! { std::string::String }
        }
    }
}
