use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::Type;

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

    pub fn actual_response_type(&self, response_ty: Type) -> TokenStream2 {
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

/// Generates a type safe asynchronous function that calls the input ItemFn
pub fn generate(input: &syn::ItemFn, impl_kind: RequestImpl) -> TokenStream2 {
    let (vis, fn_name) = extract_fn_info(input);
    let client_callback = impl_kind.name(fn_name.span());
    let fn_args = extract_fn_args(input);
    let arg_idents = extract_arg_idents(input);
    let response_ty = extract_response_ty(input);
    let actual_response_ty = impl_kind.actual_response_type(response_ty.clone());
    let return_response = impl_kind.return_response();
    let rust_doc = impl_kind.rust_doc();

    quote! {
        #rust_doc
        #vis async fn #client_callback(client: &::jsonrpsee::http_client::HttpClient, #(#fn_args),*) -> #actual_response_ty {
            use ::jsonrpsee::core::client::ClientT as _;

            let params = ::jsonrpsee::rpc_params!(#(#arg_idents),*);
            let response = client
                .request::<#response_ty, _>(stringify!(#fn_name), params)
                .await;

            #return_response
        }
    }
}

/// Extracts the function's visibility and name.
fn extract_fn_info(input: &syn::ItemFn) -> (&syn::Visibility, &syn::Ident) {
    (&input.vis, &input.sig.ident)
}

/// Extracts the function argument types (excluding `self`).
fn extract_fn_args(input: &syn::ItemFn) -> Vec<&syn::PatType> {
    input
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("function cannot take self"),
            syn::FnArg::Typed(pat_type) => pat_type,
        })
        .collect()
}

/// Extracts the argument identifiers for use in macro calls.
fn extract_arg_idents(input: &syn::ItemFn) -> Vec<&syn::Pat> {
    input
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("function cannot take self"),
            syn::FnArg::Typed(pat_type) => &*pat_type.pat,
        })
        .collect()
}

/// Determines the return type of the function.
fn extract_response_ty(input: &syn::ItemFn) -> Type {
    match &input.sig.output {
        syn::ReturnType::Type(_, ty) => ty.as_ref().clone(),
        syn::ReturnType::Default => syn::parse_quote! {()},
    }
}
