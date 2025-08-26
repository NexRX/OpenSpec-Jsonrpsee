use crate::rpc_method::{RpcMethodArgs, model::RpcMethod};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn generate(
    RpcMethod {
        input_vis,
        input_ident,
        fn_args_contextless,
        fn_args_contextless_as_ident,
        response_ty,
        ..
    }: &RpcMethod,
    RpcMethodArgs {
        client,
        client_field,
    }: &RpcMethodArgs,
) -> TokenStream2 {
    if client.is_none() {
        return quote! {};
    }
    let client_field = client_field.clone().unwrap_or(syn::parse_quote!(client));

    quote! {
        impl #client {
            #input_vis async fn #input_ident(&self, #fn_args_contextless) -> ::std::result::Result<#response_ty, ::jsonrpsee::core::ClientError> {
                use ::jsonrpsee::core::client::ClientT as _;

                let params = ::jsonrpsee::rpc_params!(#fn_args_contextless_as_ident);
                let response = self.#client_field
                    .request::<#response_ty, _>(stringify!(#input_ident), params)
                    .await;

                response
            }
        }
    }
}
