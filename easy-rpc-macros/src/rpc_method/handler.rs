use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

pub fn generate(
    input: &syn::ItemFn,
    original_ident: &Ident,
    context: &syn::Type,
    response: &syn::Type,
) -> TokenStream2 {
    let original_fn = original_ident;
    let arguments_parse_impl = gen_arguments_parse_impl(input);
    let arguments_supply = gen_arguments_supply(input);

    quote::quote! {
        fn handler(&self) -> SyncCallback<#context, ::jsonrpsee::core::RpcResult<#response>> {
            fn callback_wrapper<'a, 'b, 'c>(
                params: ::jsonrpsee::types::Params<'a>,
                _context: &'b #context,
                _ext: &'c ::jsonrpsee::Extensions,
            ) -> ::jsonrpsee::core::RpcResult<#response> {
                #arguments_parse_impl
                let response = #original_fn(#arguments_supply);
                Ok(response)
            }

            callback_wrapper
        }
    }
}

fn gen_arguments_parse_impl(input: &syn::ItemFn) -> TokenStream2 {
    use syn::{FnArg, Pat, PatIdent};

    // Collect argument names and types, skipping the receiver if present
    let args: Vec<_> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let pat = &*pat_type.pat;
                let ident = match pat {
                    Pat::Ident(PatIdent { ident, .. }) => {
                        quote::quote! { #ident }
                    }
                    _ => quote::quote! { _ },
                };
                let ty = &*pat_type.ty;
                Some((ident, quote::quote! { #ty }))
            } else {
                None
            }
        })
        .collect();

    let (pat, ty) = if args.is_empty() {
        (quote::quote! { () }, quote::quote! { () })
    } else {
        let idents = args.iter().map(|(ident, _)| ident.clone());
        let tys = args.iter().map(|(_, ty)| ty.clone());
        (
            quote::quote! { ( #(#idents),* ) },
            quote::quote! { ( #(#tys),* ) },
        )
    };

    let parse_fn = if args.len() == 1 {
        quote::quote! { one }
    } else {
        quote::quote! { parse }
    };

    quote::quote! {
        let #pat : #ty = params.#parse_fn()?;
    }
}

fn gen_arguments_supply(input: &syn::ItemFn) -> TokenStream2 {
    use syn::{FnArg, Pat, PatIdent};

    // Collect argument names and types, skipping the receiver if present
    let args: Vec<_> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                let pat = &*pat_type.pat;
                let ident = match pat {
                    Pat::Ident(PatIdent { ident, .. }) => {
                        quote::quote! { #ident }
                    }
                    _ => quote::quote! { _ },
                };
                let ty = &*pat_type.ty;
                Some((ident, quote::quote! { #ty }))
            } else {
                None
            }
        })
        .collect();

    if args.is_empty() {
        quote::quote! {}
    } else {
        let idents = args.iter().map(|(ident, _)| ident.clone());
        quote::quote! { #(#idents),* }
    }
}
