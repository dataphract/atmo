use atmo::nsid;
use atmo_lexicon::{FieldSchema, Object};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};

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
