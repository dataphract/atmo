use std::collections::{btree_map, BTreeMap};

use atmo_core::nsid::FullReference;
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
            #[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
        let crate_ = crate::crate_name();
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

        let union_into_des_json = quote! {
            serde::de::value::MapDeserializer::new(
                union_.map
                    .iter()
                    .map(|(k, v)| (k.as_ref(), v))
            )
        };

        let union_into_des_cbor = quote! {
            serde::de::value::MapDeserializer::new(
                union_.map
                    .iter()
                    .map(|(k, v)| (k.as_ref(), atmo_core::union_::IpldIntoDeserializer(v.clone())))
            )
        };

        let mut known_tags = Vec::new();
        let mut match_cases = Vec::new();

        for (name, variant) in variant_names.iter() {
            let full_tag = variant.nsid.to_string();
            let tag = full_tag.strip_suffix("#main").unwrap_or(full_tag.as_str());

            known_tags.push(tag.to_owned());

            let ident = quote::format_ident!("{name}");
            let inner = &variant.path;

            let map_fn = if variant.needs_boxed {
                quote! { |val| Self::#ident(std::boxed::Box::new(val)) }
            } else {
                quote! { Self::#ident }
            };

            match_cases.push(quote! {
                #tag => #inner::deserialize(map_des).map(#map_fn)
            });
        }

        let other_variant = self.is_open.then(|| {
            quote! {
                #[serde(untagged)]
                Other(#crate_::Unknown)
            }
        });

        let other_case = if self.is_open {
            quote! {
                _ => #crate_::Unknown::deserialize(map_des).map(Self::Other)
            }
        } else {
            quote! {
                other => return Err(D::Error::unknown_variant(other, &[
                    #(#known_tags,)*
                ]))
            }
        };

        quote! {
            #(#[doc = #doc])*
            #[derive(Debug, serde::Serialize)]
            pub enum #ident {
                #(#variants,)*
                #other_variant
            }

            impl<'de> serde::Deserialize<'de> for #ident {
                fn deserialize<D>(des: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    use serde::de::Error as _;

                    if des.is_human_readable() {
                        // Deserialize JSON field values to Value.
                        let visitor: #crate_::union_::UnionVisitor<serde_json::Value> = Default::default();
                        let union_ = des.deserialize_map(visitor)?;
                        let map_des = #union_into_des_json;

                        let res = match union_.ty.as_ref() {
                            #(#match_cases,)*
                            #other_case,
                        };

                        res.map_err(D::Error::custom)
                    } else {
                        // Deserialize CBOR field values to Ipld.
                        let visitor: #crate_::union_::UnionVisitor<ipld_core::ipld::Ipld> = Default::default();
                        let union_ = des.deserialize_map(visitor)?;
                        let map_des = #union_into_des_cbor;

                        let res = match union_.ty.as_ref() {
                            #(#match_cases,)*
                            #other_case,
                        };

                        res.map_err(D::Error::custom)
                    }
                }
            }
        }
        .to_tokens(tokens)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnionEnumVariant {
    pub nsid: FullReference,
    pub path: ItemPath,
    pub needs_boxed: bool,
}
