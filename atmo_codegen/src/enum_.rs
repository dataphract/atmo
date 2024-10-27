use std::collections::{btree_map, BTreeMap};

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
                #[serde(untagged)]
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

        // Resolve variant naming conflicts.
        let mut variant_names = BTreeMap::new();
        let mut num_ancestors = 0;

        'another_ancestor: loop {
            for v in &self.variants {
                let variant_name = v.path.variant_name(num_ancestors);

                let btree_map::Entry::Vacant(vacant) = variant_names.entry(variant_name.clone())
                else {
                    variant_names.clear();
                    num_ancestors += 1;

                    continue 'another_ancestor;
                };

                vacant.insert(v);
            }

            break;
        }

        let variants = variant_names.iter().map(|(name, var)| {
            let ident = quote::format_ident!("{name}");
            let ty = &var.path;

            if var.needs_boxed {
                quote! { #ident(std::boxed::Box<#ty>) }
            } else {
                quote! { #ident(#ty) }
            }
        });

        let other_variant = self.is_open.then(|| {
            quote! {
                #[serde(other)]
                Other
            }
        });

        quote! {
            #(#[doc = #doc])*
            #[derive(serde::Deserialize, serde::Serialize)]
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
    pub path: ItemPath,
    pub needs_boxed: bool,
}
