use super::RpcMethod;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::Type;

/// Generates a type safe asynchronous function that calls the input ItemFn
pub fn generate(
    RpcMethod {
        input_vis,
        input_ident,
        fn_args_contextless,
        fn_args_contextless_as_ident,
        response_ty,
        ..
    }: &RpcMethod,
    impl_kind: RequestImpl,
) -> TokenStream2 {
    let request_ident = impl_kind.name(input_ident.span());
    let actual_response_ty = impl_kind.actual_response_type(&response_ty);
    let return_response = impl_kind.return_response();
    let rust_doc = impl_kind.rust_doc();

    quote! {
        #rust_doc
        #input_vis async fn #request_ident(client: &::jsonrpsee::http_client::HttpClient, #fn_args_contextless) -> #actual_response_ty {
            use ::jsonrpsee::core::client::ClientT as _;

            let params = ::jsonrpsee::rpc_params!(#fn_args_contextless_as_ident);
            let response = client
                .request::<#response_ty, _>(stringify!(#input_ident), params)
                .await;

            #return_response
        }
    }
}

pub enum RequestImpl {
    Checked,
    Unchecked,
}

impl RequestImpl {
    pub fn name(&self, fn_span: Span) -> syn::Ident {
        syn::Ident::new(
            match self {
                RequestImpl::Unchecked => "request_unchecked",
                RequestImpl::Checked => "request",
            },
            fn_span,
        )
    }

    pub fn actual_response_type(&self, response_ty: &Type) -> TokenStream2 {
        match self {
            RequestImpl::Unchecked => quote! { #response_ty },
            RequestImpl::Checked => {
                quote! { ::std::result::Result<#response_ty, ::jsonrpsee::core::ClientError> }
            }
        }
    }

    pub fn return_response(&self) -> TokenStream2 {
        match self {
            RequestImpl::Unchecked => {
                quote! { response.expect("RPC call failed but should need successful response") }
            }
            RequestImpl::Checked => quote! { response },
        }
    }

    pub fn rust_doc(&self) -> TokenStream2 {
        match self {
            RequestImpl::Unchecked => {
                quote! { #[doc = "Makes a type safe RPC request with the given client. This function will panic if the RPC call fails to call the implementation."] }
            }
            RequestImpl::Checked => {
                quote! { #[doc = "Makes a type safe RPC request with the given client"] }
            }
        }
    }
}
