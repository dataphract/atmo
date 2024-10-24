use std::collections::HashSet;

use heck::{ToPascalCase, ToSnakeCase};
use atmo::lexicon::{FieldSchema, Object};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

use crate::emit_string_type;

pub fn emit_struct(name: &str, object: &Object) -> StructDef {
    // Convert name to PascalCase.
    let typename = quote::format_ident!("{}", name.to_pascal_case());

    let required: HashSet<_> = object.required.iter().map(String::as_str).collect();
    let nullable: HashSet<_> = object.nullable.iter().map(String::as_str).collect();

    let mut fields = Vec::new();
    for (prop_name, prop_schema) in &object.properties {
        let field = emit_struct_field(
            prop_name,
            prop_schema,
            required.contains(prop_name.as_str()),
            nullable.contains(prop_name.as_str()),
        );
        fields.push(field);
    }

    StructDef {
        name: typename,
        fields,
    }
}

pub fn emit_struct_field(
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

    let field_ident = syn::Ident::new(&field_name, Span::call_site());

    let inner_ty: TokenStream = match schema {
        FieldSchema::Array(a) => {
            let elem_ty = match &*a.items {
                FieldSchema::CidLink => quote! { CidLink },
                FieldSchema::Ref(_) => quote! { () },
                FieldSchema::String(_) => quote! { String },
                FieldSchema::Union(_) => quote! { Union },
                FieldSchema::Unknown => quote! { Unknown },
                x => panic!("unhandled array element type: {x:?}"),
            };

            quote! { Vec<#elem_ty> }
        }

        FieldSchema::Blob(b) => quote::format_ident!("Blob").to_token_stream(),

        FieldSchema::Boolean(b) => {
            desc = b.description.clone();
            quote::format_ident!("bool").to_token_stream()
        }

        FieldSchema::Bytes(b) => {
            desc = b.description.clone();
            quote! { Vec<u8> }
        }

        FieldSchema::CidLink => {
            eprintln!("cid-link not implemented yet");
            quote! { () }
        }

        FieldSchema::Integer(i) => {
            // TODO: deranged
            if i.minimum.is_some() || i.maximum.is_some() {
                eprintln!("ranged integer support not implemented yet sorry lol");
            }

            quote! { i64 }
        }

        FieldSchema::Ref(_) => {
            eprintln!("Ref resolution not implemented yet");
            quote! { () }
        }

        FieldSchema::String(s) => emit_string_type(s),

        FieldSchema::Union(u) => {
            eprintln!("unions not properly implemented yet");

            eprintln!("union refs: {:?}", &u.refs);

            quote! { () }
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

pub struct StructDef {
    pub name: syn::Ident,
    pub fields: Vec<Field>,
}

impl ToTokens for StructDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields = self.fields.iter();

        quote! {
            pub struct #name {
                #(#fields),*
            }
        }
        .to_tokens(tokens)
    }
}

pub struct Field {
    pub doc: Option<String>,
    pub name: syn::Ident,
    pub optional: bool,
    pub nullable: bool,
    pub inner_ty: TokenStream,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = crate::crate_name();

        let doc_attr = self.doc.clone().map(FieldAttr::Doc);
        let skip_none = self.optional.then_some(FieldAttr::SerdeSkipNone);
        let attrs = doc_attr.iter().chain(skip_none.as_ref());

        let name = &self.name;

        let inner_ty = &self.inner_ty;
        let ty = match (self.optional, self.nullable) {
            (false, false) => quote! { #inner_ty },
            (false, true) => quote! { #crate_::Nullable<#inner_ty> },
            (true, false) => quote! { std::option::Option<#inner_ty> },
            (true, true) => quote! { std::option::Option<#crate_::Nullable<#inner_ty>> },
        };

        quote! {
            #(#attrs)*
            pub #name: #ty
        }
        .to_tokens(tokens)
    }
}

enum FieldAttr {
    Doc(String),
    SerdeSkipNone,
}

impl ToTokens for FieldAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tt = match self {
            FieldAttr::Doc(s) => quote! { #[doc = #s] },
            FieldAttr::SerdeSkipNone => {
                quote! { #[serde(skip_serializing_if = "std::option::Option::is_none")] }
            }
        };

        tt.to_tokens(tokens)
    }
}
