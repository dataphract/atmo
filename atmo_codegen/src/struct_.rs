use std::iter;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::Type;

#[derive(Debug)]
pub struct RustStructDef {
    pub name: syn::Ident,
    pub fields: Vec<RustStructField>,
}

impl ToTokens for RustStructDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let fields = self.fields.iter();

        quote! {
            #[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
            pub struct #name {
                #(#fields,)*
            }
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug)]
pub struct RustStructField {
    pub doc: Option<String>,
    pub name: syn::Ident,
    pub rename: String,
    pub is_array: bool,
    pub is_optional: bool,
    pub is_nullable: bool,
    pub inner_ty: Type,
}

impl ToTokens for RustStructField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_ = crate::crate_name();

        let doc_attr = self.doc.clone().map(FieldAttr::Doc);
        let serde_default = self.is_optional.then_some(FieldAttr::SerdeDefault);
        let serde_rename =
            (self.name != self.rename).then(|| FieldAttr::SerdeRename(self.rename.clone()));
        let serde_skip_none = self.is_optional.then_some(FieldAttr::SerdeSkipNone);
        let serde_with = match self.inner_ty {
            Type::Bytes => {
                let path = if self.is_optional {
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
            .chain(serde_rename.as_ref())
            .chain(serde_skip_none.as_ref())
            .chain(serde_with.as_ref());

        let name = &self.name;

        let inner_ty = self.inner_ty.clone();
        let maybe_arr_ty = if self.is_array {
            Type::Vec(inner_ty.into())
        } else {
            inner_ty
        };
        let ty = match (self.is_optional, self.is_nullable) {
            (false, false) => maybe_arr_ty,
            (false, true) => Type::Nullable(maybe_arr_ty.into()),
            (true, false) => Type::Option(maybe_arr_ty.into()),
            (true, true) => Type::Option(Type::Nullable(maybe_arr_ty.into()).into()),
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
