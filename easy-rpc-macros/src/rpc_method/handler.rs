use proc_macro2::TokenStream as TokenStream2;
use syn::{Ident, spanned::Spanned as _};

pub fn generate(
    input: &syn::ItemFn,
    original_ident: &Ident,
    context_ty: &syn::Type,
    context_ty_to_ref: &syn::Type,
    response: &syn::Type,
) -> TokenStream2 {
    let original_fn = original_ident;
    let arguments_parse_impl = gen_arguments_parse_impl(input);
    let arguments_supply = gen_arguments_supply(input);
    let context_ident =
        extract_context_ident(input).unwrap_or_else(|| Ident::new("_context", input.span()));

    quote::quote! {
        fn handler(&self) -> SyncCallback<#context_ty, ::jsonrpsee::core::RpcResult<#response>> {
            fn callback_wrapper<'a, 'b, 'c>(
                params: ::jsonrpsee::types::Params<'a>,
                #context_ident: &'b #context_ty_to_ref,
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
                // Skip argument if it has #[context] attribute
                if pat_type
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("context"))
                {
                    return None;
                }
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

pub fn extract_context_ident(input: &syn::ItemFn) -> Option<syn::Ident> {
    for arg in &input.sig.inputs {
        if let syn::FnArg::Typed(pat_type) = arg {
            for attr in &pat_type.attrs {
                if attr.path().is_ident("context") {
                    if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = &*pat_type.pat {
                        return Some(ident.clone());
                    }
                }
            }
        }
    }
    None
}
