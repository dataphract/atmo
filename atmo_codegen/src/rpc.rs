use atmo_core::nsid::Nsid;
use quote::{quote, ToTokens};

use crate::module::ItemPath;

#[derive(Debug)]
pub struct RustRpcDef {
    pub name: syn::Ident,
    pub ty: RpcType,
    pub nsid: Nsid,
    pub params: Option<ItemPath>,
    pub input: Option<ItemPath>,
    pub output: Option<ItemPath>,
    pub output_encoding: String,
    pub error: Option<ItemPath>,
}

impl ToTokens for RustRpcDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_ = crate::crate_name();
        let ident = &self.name;

        let params = self
            .params
            .as_ref()
            .map(|p| p.into_token_stream())
            .unwrap_or(quote! { #crate_::Nothing });

        let input = self
            .input
            .as_ref()
            .map(|i| i.into_token_stream())
            .unwrap_or(quote! { #crate_::Nothing });

        let output = self
            .output
            .as_ref()
            .map(|o| o.into_token_stream())
            .unwrap_or(quote! { #crate_::Nothing });

        let error = self
            .error
            .as_ref()
            .map(|e| quote! { #e })
            .unwrap_or(quote! { String });

        let method = match self.ty {
            RpcType::Query => quote! { http::Method::GET },
            RpcType::Procedure => quote! { http::Method::POST },
        };

        let nsid = &self.nsid.as_str();
        let output_encoding = &self.output_encoding;

        quote! {
            #[derive(Debug)]
            pub struct #ident;

            impl #crate_::xrpc::Request for #ident {
                type Params = #params;
                type Input = #input;
                type Output = #output;
                type Error = #error;

                #[inline]
                fn method() -> http::Method {
                    #method
                }

                #[inline]
                fn nsid() -> &'static str {
                    #nsid
                }

                #[inline]
                fn output_encoding() -> &'static str {
                    #output_encoding
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RpcType {
    Query,
    Procedure,
}
