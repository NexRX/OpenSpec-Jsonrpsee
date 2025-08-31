use super::model::RpcMethod;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Ident, PatType, punctuated::Punctuated, token::Comma};

pub fn generate(model: &RpcMethod) -> TokenStream2 {
    let fn_input = &model.input_ident;
    let arguments_parse_impl = gen_arguments_parse_impl(&model.fn_args_contextless);
    let context_ident = model
        .context_ident
        .clone()
        .unwrap_or_else(|| Ident::new("_context", model.input_span.clone()));

    if model.input_async.is_some() {
        generate_async_handler(model, &context_ident, &arguments_parse_impl, &fn_input)
    } else {
        generate_sync_handler(model, &context_ident, &arguments_parse_impl, &fn_input)
    }
}

fn generate_async_handler(
    RpcMethod {
        context_ty_owned,
        context_ident: ctx,
        context_ty_referenced,
        response_ty,
        fn_args_as_ident,
        ..
    }: &RpcMethod,
    context_ident: &Ident,
    arguments_parse_impl: &TokenStream2,
    fn_input: &Ident,
) -> TokenStream2 {
    let fn_args_stream = fn_args_as_ident
        .iter()
        .map(|arg| {
            if ctx.as_ref().is_some_and(|ctx| ctx == arg) {
                if *context_ty_referenced {
                    quote::quote! { &#arg }
                } else {
                    quote::quote! { (*#arg).clone() }
                }
            } else {
                quote::quote! { #arg }
            }
        })
        .collect::<Vec<_>>();
    let fn_args_stream = quote! { #(#fn_args_stream),* };

    quote::quote! {
        #[allow(clippy::ptr_arg)] // Suppressed due to complexity in generating for all context types
        fn handler(&self) -> ::openspec_jsonrpsee::ServerHandler<#context_ty_owned, ::jsonrpsee::core::RpcResult<#response_ty>> {
            fn callback_wrapper(
                params: ::jsonrpsee::types::Params<'static>,
                #context_ident: ::std::sync::Arc<#context_ty_owned>,
                _ext: ::jsonrpsee::Extensions,
            ) -> ::std::pin::Pin<
                Box<dyn ::std::future::Future<Output = ::jsonrpsee::core::RpcResult<#response_ty>> + Send>,
            > {
                Box::pin(async move {
                    #arguments_parse_impl
                    let response = #fn_input(#fn_args_stream).await;
                    Ok(response)
                })
            }

            ::openspec_jsonrpsee::ServerHandler::Async(callback_wrapper)
        }
    }
}

fn generate_sync_handler(
    RpcMethod {
        context_ty_owned,
        response_ty,
        fn_args_as_ident,
        ..
    }: &RpcMethod,
    context_ident: &Ident,
    arguments_parse_impl: &TokenStream2,
    fn_input: &Ident,
) -> TokenStream2 {
    quote::quote! {
        #[allow(clippy::ptr_arg)] // Suppressed due to complexity in generating for all context types
        fn handler(&self) -> ::openspec_jsonrpsee::ServerHandler<#context_ty_owned, ::jsonrpsee::core::RpcResult<#response_ty>> {
            fn callback_wrapper<'a, 'b, 'c>(
                params: ::jsonrpsee::types::Params<'a>,
                #context_ident: &'b #context_ty_owned,
                _ext: &'c ::jsonrpsee::Extensions,
            ) -> ::jsonrpsee::core::RpcResult<#response_ty> {
                #arguments_parse_impl
                let response = #fn_input(#fn_args_as_ident);
                Ok(response)
            }

            ::openspec_jsonrpsee::ServerHandler::Sync(callback_wrapper)
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

    // Determine the appropriate parsing method based on the number of arguments
    let fn_parse = match fn_args_contextless.len() {
        1 => quote::quote! { one },
        _ => quote::quote! { parse },
    };

    quote::quote! {
        let (#pat): (#ty) = params.#fn_parse()?;
    }
}
