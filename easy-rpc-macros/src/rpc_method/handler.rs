use super::model::RpcMethod;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Ident, PatType, punctuated::Punctuated, token::Comma};

pub fn generate(
    RpcMethod {
        input_async,
        input_ident,
        context_ty_owned,
        context_ident,
        fn_args_as_ident,
        fn_args_contextless,
        response_ty,
        input_span,
        ..
    }: &RpcMethod,
) -> TokenStream2 {
    let fn_input = input_ident;
    let arguments_parse_impl = gen_arguments_parse_impl(&fn_args_contextless);
    let context_ident = context_ident
        .clone()
        .unwrap_or(Ident::new("_context", input_span.clone()));

    if input_async.is_some() {
        quote::quote! {
            #[allow(clippy::ptr_arg)] // Reason: too hard to generate for all the context types
            fn handler(&self) -> ::easy_rpc::AsyncCallback<#context_ty_owned, ::jsonrpsee::core::RpcResult<#response_ty>> {
                fn callback_wrapper(
                    params: ::jsonrpsee::types::Params<'static>,
                    #context_ident: ::std::sync::Arc<#context_ty_owned>,
                    _ext: ::jsonrpsee::Extensions,
                ) -> ::std::pin::Pin<
                    Box<dyn ::std::future::Future<Output = ::jsonrpsee::core::RpcResult<#response_ty>> + Send>,
                > {
                    Box::pin(async move {
                        #arguments_parse_impl
                        let response = #fn_input(#fn_args_as_ident).await;
                        Ok(response)
                    })
                }

                callback_wrapper
            }
        }
    } else {
        quote::quote! {
            #[allow(clippy::ptr_arg)] // Reason: too hard to generate for all the context types
            fn handler(&self) -> ::easy_rpc::SyncCallback<#context_ty_owned, ::jsonrpsee::core::RpcResult<#response_ty>> {
                fn callback_wrapper<'a, 'b, 'c>(
                    params: ::jsonrpsee::types::Params<'a>,
                    #context_ident: &'b #context_ty_owned,
                    _ext: &'c ::jsonrpsee::Extensions,
                ) -> ::jsonrpsee::core::RpcResult<#response_ty> {
                    #arguments_parse_impl
                    let response = #fn_input(#fn_args_as_ident);
                    Ok(response)
                }

                callback_wrapper
            }
        }
    }
}

fn gen_arguments_parse_impl(fn_args_contextless: &Punctuated<PatType, Comma>) -> TokenStream2 {
    use syn::{Pat, Type};

    let pat: Punctuated<Pat, Comma> = fn_args_contextless
        .iter()
        .map(|pat_type| (*pat_type.pat).clone())
        .collect();

    let ty: Punctuated<Type, Comma> = fn_args_contextless
        .iter()
        .map(|pat_type| (*pat_type.ty).clone())
        .collect();

    let fn_parse = if fn_args_contextless.len() == 1 {
        quote::quote! { one }
    } else {
        quote::quote! { parse }
    };

    quote::quote! {
        let (#pat): (#ty) = params.#fn_parse()?;
    }
}
