use std::iter;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::Type;

#[derive(Debug)]
pub struct StructDef {
    pub name: syn::Ident,
    pub fields: Vec<Field>,
}

impl ToTokens for StructDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields = self.fields.iter();

        quote! {
            #[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
            pub struct #name {
                #(#fields),*
            }
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug)]
pub struct Field {
    pub doc: Option<String>,
    pub name: syn::Ident,
    pub rename: String,
    pub optional: bool,
    pub nullable: bool,
    pub inner_ty: Type,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = crate::crate_name();

        let doc_attr = self.doc.clone().map(FieldAttr::Doc);
        let serde_default = self.optional.then_some(FieldAttr::SerdeDefault);
        let serde_rename = FieldAttr::SerdeRename(self.rename.clone());
        let serde_skip_none = self.optional.then_some(FieldAttr::SerdeSkipNone);
        let serde_with = match self.inner_ty {
            Type::Bytes => {
                let path = if self.optional {
                    format!("{crate_}::bytes::serde::option")
                } else {
                    format!("{crate_}::bytes::serde")
                };

                Some(FieldAttr::SerdeWith(path))
            }
            _ => None,
        };
        let attrs = doc_attr
            .iter()
            .chain(serde_default.as_ref())
            .chain(iter::once(&serde_rename))
            .chain(serde_skip_none.as_ref())
            .chain(serde_with.as_ref());

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
    SerdeDefault,
    SerdeRename(String),
    SerdeSkipNone,
    SerdeWith(String),
}

impl ToTokens for FieldAttr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tt = match self {
            FieldAttr::Doc(s) => quote! { #[doc = #s] },
            FieldAttr::SerdeDefault => quote! { #[serde(default)] },
            FieldAttr::SerdeRename(s) => quote! { #[serde(rename = #s)] },
            FieldAttr::SerdeSkipNone => {
                quote! { #[serde(skip_serializing_if = "std::option::Option::is_none")] }
            }
            FieldAttr::SerdeWith(s) => {
                quote! { #[serde(with = #s)] }
            }
        };

        tt.to_tokens(tokens)
    }
}
