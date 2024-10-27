use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::module::ItemPath;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringEnumDef {
    pub ident: syn::Ident,
    pub variants: Vec<StringEnumVariant>,
    pub is_open: bool,
}

impl ToTokens for StringEnumDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let variants = self.variants.iter();

        let other_variant = self.is_open.then(|| {
            quote! {
                #[serde(other)]
                Other(String),
            }
        });

        quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            pub enum #ident {
                #(#variants,)*
                #other_variant
            }
        }
        .to_tokens(tokens)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringEnumVariant {
    pub doc: Option<String>,
    pub string_value: String,
    pub ident: syn::Ident,
}

impl ToTokens for StringEnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc = self.doc.iter();
        let ident = &self.ident;
        let string_value = &self.string_value;

        quote! {
            #(#[doc = #doc])*
            #[serde(rename = #string_value)]
            #ident
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnionEnumDef {
    pub doc: Option<String>,
    pub ident: syn::Ident,
    pub variants: Vec<UnionEnumVariant>,
    pub is_open: bool,
}

impl ToTokens for UnionEnumDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc = self.doc.iter();
        let ident = &self.ident;
        let variants = self.variants.iter();

        let other_variant = self.is_open.then(|| {
            quote! {
                #[serde(other)]
                Other()
            }
        });

        quote! {
            #(#[doc = #doc])*
            pub enum #ident {
                #(#variants,)*
                #other_variant
            }
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnionEnumVariant {
    pub name: syn::Ident,
    pub inner: ItemPath,
}

impl ToTokens for UnionEnumVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let inner = &self.inner;

        quote! {
            #name(#inner)
        }
        .to_tokens(tokens)
    }
}
