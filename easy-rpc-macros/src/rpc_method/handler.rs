use super::model::RpcMethod;
use proc_macro2::TokenStream as TokenStream2;
use syn::{Ident, PatType, Type, punctuated::Punctuated, token::Comma};

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
        .unwrap_or_else(|| Ident::new("_context", input_span.clone()));

    if input_async.is_some() {
        generate_async_handler(
            context_ty_owned,
            &response_ty,
            &context_ident,
            &arguments_parse_impl,
            fn_input,
            fn_args_as_ident,
        )
    } else {
        generate_sync_handler(
            context_ty_owned,
            &response_ty,
            &context_ident,
            &arguments_parse_impl,
            fn_input,
            fn_args_as_ident,
        )
    }
}

fn generate_async_handler(
    context_ty_owned: &Type,
    response_ty: &Type,
    context_ident: &Ident,
    arguments_parse_impl: &TokenStream2,
    fn_input: &Ident,
    fn_args_as_ident: &Punctuated<Ident, Comma>,
) -> TokenStream2 {
    quote::quote! {
        #[allow(clippy::ptr_arg)] // Suppressed due to complexity in generating for all context types
        fn handler(&self) -> ::easy_rpc::ServerHandler<#context_ty_owned, ::jsonrpsee::core::RpcResult<#response_ty>> {
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

            ::easy_rpc::ServerHandler::Async(callback_wrapper)
        }
    }
}

fn generate_sync_handler(
    context_ty_owned: &Type,
    response_ty: &Type,
    context_ident: &Ident,
    arguments_parse_impl: &TokenStream2,
    fn_input: &Ident,
    fn_args_as_ident: &Punctuated<Ident, Comma>,
) -> TokenStream2 {
    quote::quote! {
        #[allow(clippy::ptr_arg)] // Suppressed due to complexity in generating for all context types
        fn handler(&self) -> ::easy_rpc::ServerHandler<#context_ty_owned, ::jsonrpsee::core::RpcResult<#response_ty>> {
            fn callback_wrapper<'a, 'b, 'c>(
                params: ::jsonrpsee::types::Params<'a>,
                #context_ident: &'b #context_ty_owned,
                _ext: &'c ::jsonrpsee::Extensions,
            ) -> ::jsonrpsee::core::RpcResult<#response_ty> {
                #arguments_parse_impl
                let response = #fn_input(#fn_args_as_ident);
                Ok(response)
            }

            ::easy_rpc::ServerHandler::Sync(callback_wrapper)
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
