use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use heck::ToSnakeCase;
use module::{ModulePath, Modules};
use neophron::{
    lexicon::{Schema, StringFormat},
    nsid::Nsid,
    Lexicon,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

mod module;
mod structs;
mod unions;

fn crate_name() -> syn::Ident {
    quote::format_ident!("neophron")
}

/// ATProto bindings generator.
#[derive(Default)]
pub struct Gen {
    // Input Lexicon schemae.
    lexicons: BTreeMap<Nsid, Lexicon>,
    // Map from fully-qualified Lexicon NSIDs to schema items.
    namespaces: BTreeMap<String, HashSet<Item>>,
    definitions: BTreeMap<Nsid, ()>,
}

impl Gen {
    pub fn new() -> Gen {
        Gen::default()
    }

    pub fn add_lexicon(&mut self, source: &str) {
        let mut des = serde_json::Deserializer::from_str(source);
        let lex: Lexicon = serde_path_to_error::deserialize(&mut des).unwrap();

        let nsid = Nsid::from_str(lex.id.as_str()).unwrap();

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

        let mut modules = Modules::default();

        for (nsid, lex) in &self.lexicons {
            for (name, schema) in &lex.defs {
                let mut module_path = ModulePath::new();
                for segment in nsid.segments() {
                    module_path.push(segment.to_snake_case());
                }

                modules.ensure_exists(module_path);

                let def_path = name.to_snake_case();

                match schema {
                    Schema::Array(_) => {}

                    Schema::Object(o) => {
                        let tokens = crate::structs::emit_struct(name, o).into_token_stream();
                        println!("{tokens}");
                    }

                    Schema::Record(r) => {
                        //let tokens = emit_struct("Record", &r.record);

                        // println!("{tokens}");
                    }

                    Schema::Procedure(_) | Schema::Query(_) => (),

                    Schema::String(s) => {
                        eprintln!("{def_path}: {s:?}");
                    }

                    Schema::Subscription(_) => (),

                    Schema::Token => (),

                    s => panic!("unhandled top-level definition: {s:?}"),
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

pub fn emit_string_type(s: &neophron::lexicon::String) -> TokenStream {
    let Some(format) = &s.format else {
        return quote! { std::string::String };
    };

    let crate_ = crate_name();

    match format {
        StringFormat::Datetime => quote! { #crate_::datetime::DateTimeString },
        StringFormat::Did => quote! { #crate_::did::Did },
        StringFormat::Handle => quote! { #crate_::handle::Handle },
        StringFormat::Language => quote! { std::string::String },
        StringFormat::Nsid => quote! { #crate_::nsid::Nsid },
        StringFormat::RecordKey => quote! { #crate_::rkey::RecordKey },
        StringFormat::Tid => quote! { #crate_::tid::Tid },
        _ => {
            eprintln!("string format `{format:?}` not handled yet");
            quote! { std::string::String }
        }
    }
}
