use quote::quote;
use syn::{FnArg, ItemFn, LitStr, Pat, PatIdent, PatType, punctuated::Punctuated, token::Comma};

pub fn generate(input: &syn::ItemFn) -> proc_macro2::TokenStream {
    let name = input.sig.ident.to_string();
    let description = extract_description(input);
    let deprecated = extract_deprecated(input);
    let params = extract_params(input);

    quote! {
        fn spec(&self) -> ::easy_rpc::spec::Method {
            ::easy_rpc::spec::Method {
                name: stringify!(#name).into(),
                tags: None,
                summary: None,
                description: #description,
                external_docs: None,
                params: vec![#(#params),*],
                result: None,
                deprecated: #deprecated,
                servers: None,
                errors: None,
                links: None,
                param_structure: None,
                examples: None,
            }
        }
    }
}

fn extract_deprecated(input: &ItemFn) -> proc_macro2::TokenStream {
    let is_deprecated = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("deprecated"));
    quote! { Some(#is_deprecated) }
}

fn extract_description(input: &ItemFn) -> proc_macro2::TokenStream {
    let doc_lines: Vec<String> = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                attr.parse_args::<LitStr>().ok().map(|lit| lit.value())
            } else {
                None
            }
        })
        .collect();

    if doc_lines.is_empty() {
        quote! { None }
    } else {
        let joined = doc_lines.join("\n");
        quote! { Some(#joined) }
    }
}

fn extract_params(input: &ItemFn) -> Vec<proc_macro2::TokenStream> {
    filtered_params(&input.sig.inputs)
        .into_iter()
        .map(|param| {
            let name = match &param {
                FnArg::Typed(PatType { pat, .. }) => match &**pat {
                    Pat::Ident(PatIdent { ident, .. }) => quote! { stringify!(#ident).into() },
                    _ => quote! { None },
                },
                FnArg::Receiver(_) => quote! { "self".into() },
            };

            let schema = match &param {
                FnArg::Typed(PatType { ty, .. }) => quote! { schemars::schema_for!(#ty) },
                FnArg::Receiver(_) => quote! { panic!("Receiver type not supported for schema") },
            };

            let deprecated = match &param {
                FnArg::Typed(PatType { attrs, .. }) => {
                    let is_deprecated = attrs.iter().any(|attr| attr.path().is_ident("deprecated"));
                    quote! { Some(#is_deprecated) }
                }
                FnArg::Receiver(_) => quote! { None },
            };

            quote! {
                ::easy_rpc::spec::ContentDescriptor {
                    name: #name,
                    summary: None,
                    description: None,
                    required: Some(true),
                    schema: #schema,
                    deprecated: #deprecated,
                }
            }
        })
        .collect()
}

/// Remove context parameters from the input
fn filtered_params(input: &Punctuated<FnArg, Comma>) -> Vec<FnArg> {
    input
        .iter()
        .filter(|param| match param {
            FnArg::Receiver(_) => true,
            FnArg::Typed(PatType { attrs, .. }) => {
                attrs.iter().all(|attr| !attr.path().is_ident("context"))
            }
        })
        .cloned()
        .collect()
}
