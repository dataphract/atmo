use atmo_core::nsid::Nsid;
use quote::{quote, ToTokens};

use crate::module::ItemPath;

#[derive(Debug)]
pub struct RustRpcDef {
    pub name: syn::Ident,
    pub ty: RpcType,
    pub nsid: Nsid,
    pub params: Option<ItemPath>,
    pub input: Option<RustRpcIo>,
    pub output: Option<RustRpcIo>,
    pub error: Option<ItemPath>,
}

impl ToTokens for RustRpcDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let crate_ = crate::crate_name();
        let ident = &self.name;

        let mut params_ty = quote! { () };
        let mut serialize_params = quote! {
            let _ = params;
            Ok(String::new())
        };
        let mut deserialize_params = quote! {
            let _ = query;
            Ok(())
        };

        let mut input_ty = quote! { () };
        let mut input_err = quote! { std::convert::Infallible };
        let mut input_content_type = quote! { None };
        let mut serialize_input = quote! {
            let _ = input;
            Ok(bytes::Bytes::new())
        };
        let mut deserialize_input = quote! {
            let _ = bytes;
            Ok(())
        };

        let mut output_ty = quote! { () };
        let mut output_err = quote! { std::convert::Infallible };
        let mut _output_content_type = quote! { None };
        let mut serialize_output = quote! {
            let _ = output;
            Ok(bytes::Bytes::new())
        };
        let mut deserialize_output = quote! {
            let _ = bytes;
            Ok(())
        };

        if let Some(p) = &self.params {
            params_ty = p.to_token_stream();

            serialize_params = quote! {
                serde_urlencoded_xrpc::to_string(params)
            };

            deserialize_params = quote! {
                serde_urlencoded_xrpc::from_str(query)
            }
        }

        if let Some(i) = &self.input {
            input_ty = i.to_token_stream();

            input_err = match i {
                RustRpcIo::Bytes => quote! { std::convert::Infallible },
                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! { serde_json::Error },
            };

            input_content_type = match i {
                RustRpcIo::Bytes => quote! { None },
                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! { Some("application/json") },
            };

            serialize_input = match i {
                RustRpcIo::Bytes => quote! {
                    Ok(input.clone())
                },

                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! {
                    serde_json::to_vec(input).map(bytes::Bytes::from)
                },
            };

            deserialize_input = match i {
                RustRpcIo::Bytes => quote! {
                    Ok(bytes.clone())
                },

                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! {
                    serde_json::from_slice(bytes)
                },
            };
        }

        if let Some(o) = &self.output {
            output_ty = o.to_token_stream();

            _output_content_type = match o {
                RustRpcIo::Bytes => quote! { None },
                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! { Some("application/json") },
            };

            output_err = match o {
                RustRpcIo::Bytes => quote! { std::convert::Infallible },
                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! { serde_json::Error },
            };

            serialize_output = match o {
                RustRpcIo::Bytes => quote! {
                    Ok(output.clone())
                },

                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! {
                    serde_json::to_vec(output).map(bytes::Bytes::from)
                },
            };

            deserialize_output = match o {
                RustRpcIo::Bytes => quote! {
                    Ok(bytes.clone())
                },

                RustRpcIo::Def(_) | RustRpcIo::Ref(_) => quote! {
                    serde_json::from_slice(bytes)
                },
            };
        }

        let error_ty = self
            .error
            .as_ref()
            .map(|e| quote! { #e })
            .unwrap_or(quote! { String });

        let method = match self.ty {
            RpcType::Query => quote! { http::Method::GET },
            RpcType::Procedure => quote! { http::Method::POST },
        };

        let nsid = &self.nsid.as_str();

        quote! {
            #[derive(Debug)]
            pub struct #ident;

            impl #crate_::xrpc::Request for #ident {
                type Params = #params_ty;

                type Input = #input_ty;
                type InputError = #input_err;

                type Output = #output_ty;
                type OutputError = #output_err;

                type RpcError = #error_ty;

                #[inline]
                fn method() -> http::Method {
                    #method
                }

                #[inline]
                fn nsid() -> &'static str {
                    #nsid
                }

                fn serialize_params(
                    params: &Self::Params,
                ) -> core::result::Result<String, serde_urlencoded_xrpc::ser::Error> {
                    #serialize_params
                }

                fn deserialize_params(
                    query: &str,
                ) -> core::result::Result<Self::Params, serde_urlencoded_xrpc::de::Error> {
                    #deserialize_params
                }

                fn input_content_type() -> Option<&'static str> {
                    #input_content_type
                }

                fn serialize_input(input: &Self::Input) -> core::result::Result<bytes::Bytes, Self::InputError> {
                    #serialize_input
                }

                fn deserialize_input(bytes: &bytes::Bytes) -> core::result::Result<Self::Input, Self::InputError> {
                    #deserialize_input
                }

                fn serialize_output(output: &Self::Output) -> core::result::Result<bytes::Bytes, Self::OutputError> {
                    #serialize_output
                }

                fn deserialize_output(bytes: &bytes::Bytes) -> core::result::Result<Self::Output, Self::OutputError> {
                    #deserialize_output
                }
            }
        }
        .to_tokens(tokens);
    }
}

#[derive(Debug)]
pub enum RustRpcIo {
    Bytes,
    Def(ItemPath),
    Ref(ItemPath),
}

impl ToTokens for RustRpcIo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            RustRpcIo::Bytes => quote! { bytes::Bytes }.to_tokens(tokens),
            RustRpcIo::Def(p) | RustRpcIo::Ref(p) => quote! { #p }.to_tokens(tokens),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RpcType {
    Query,
    Procedure,
}
