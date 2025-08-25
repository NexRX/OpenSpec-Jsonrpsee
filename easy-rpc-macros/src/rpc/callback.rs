use proc_macro2::TokenStream as TokenStream2;

pub fn gen_callback_impl(
    input: &syn::ItemFn,
    wrapper_fn: &TokenStream2,
    context: &syn::Type,
    response: &syn::Type,
) -> TokenStream2 {
    let wrapped_fn = wrapper_fn;
    let arguments_parse_impl = gen_arguments_parse_impl(input);
    let arguments_supply = gen_arguments_supply(input);

    quote::quote! {
        fn callback(&self) -> SyncCallback<#context, ::jsonrpsee::core::RpcResult<#response>> {
            fn callback_wrapper<'a, 'b, 'c>(
                params: Params<'a>,
                _context: &'b #context,
                _ext: &'c Extensions,
            ) ->  ::jsonrpsee::core::RpcResult<#response>{
                println!("callback_wrapper called, {:?}", params);
                #arguments_parse_impl;
                let response = #wrapped_fn(#arguments_supply);
                Ok(response)
            }

            callback_wrapper
        }
    }
}

/// generates the parsing step in the json rpc handler
/// should create something like this when there is two arguments for example:
/// ```no_run
/// let (a, b): (String, u32) = params.parse()?;
/// ```
/// and when there is no arguments:
/// ```no_run
/// let (): () = params.parse()?;
/// ```
fn gen_arguments_parse_impl(input: &syn::ItemFn) -> TokenStream2 {
    use syn::{FnArg, Pat, PatIdent};

    // Collect argument names and types, skipping the receiver if present
    let args: Vec<_> = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            match arg {
                FnArg::Typed(pat_type) => {
                    // Get the argument name
                    let pat = &*pat_type.pat;
                    let ident = match pat {
                        Pat::Ident(PatIdent { ident, .. }) => quote::quote! { #ident },
                        _ => quote::quote! { _ },
                    };
                    let ty = &*pat_type.ty;
                    Some((ident, quote::quote! { #ty }))
                }
                _ => None,
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
            match arg {
                FnArg::Typed(pat_type) => {
                    // Get the argument name
                    let pat = &*pat_type.pat;
                    let ident = match pat {
                        Pat::Ident(PatIdent { ident, .. }) => quote::quote! { #ident },
                        _ => quote::quote! { _ },
                    };
                    let ty = &*pat_type.ty;
                    Some((ident, quote::quote! { #ty }))
                }
                _ => None,
            }
        })
        .collect();

    if args.is_empty() {
        quote::quote! {}
    } else {
        let idents = args.iter().map(|(ident, _)| ident.clone());
        quote::quote! { ( #(#idents),* ) }
    }
}
