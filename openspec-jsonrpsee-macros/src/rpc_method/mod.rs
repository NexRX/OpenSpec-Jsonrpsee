mod client;
mod handler;
mod model;
mod request;
mod sanitize_input;
mod spec;

use crate::rpc_method::request::RequestImpl;
use model::RpcMethod;
pub(crate) use model::RpcMethodArgs;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

pub fn generate_rpc_method(input: syn::ItemFn, args: RpcMethodArgs) -> TokenStream {
    let model = RpcMethod::parse(input.clone());
    let RpcMethod {
        input_ident,
        output_ident,
        context_ty_owned,
        response_ty,
        ..
    } = model.clone();

    let sanitize_input = sanitize_input::generate(&input);

    let fn_request = request::generate(&model, RequestImpl::Checked);
    let fn_request_unchecked = request::generate(&model, RequestImpl::Unchecked);

    let fn_name = gen_fn_name(&input_ident);
    let fn_spec = spec::generate(&input);
    let fn_handler = handler::generate(&model);

    let impl_client = client::generate(&model, &args);

    quote::quote! {
        #sanitize_input

        #[allow(non_camel_case_types)]
        pub struct #output_ident;

        impl #output_ident {
            #fn_request
            #fn_request_unchecked
        }

        impl ::openspec_jsonrpsee::RpcMethod<#context_ty_owned, #response_ty> for #output_ident {
            #fn_name
            #fn_spec
            #fn_handler
        }

        #impl_client
    }
    .into()
}

fn gen_fn_name(fn_name: &Ident) -> TokenStream2 {
    quote::quote! {
        fn name(&self) -> &'static str {
            stringify!(#fn_name)
        }
    }
}
