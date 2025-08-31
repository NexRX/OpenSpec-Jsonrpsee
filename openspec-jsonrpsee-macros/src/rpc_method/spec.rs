use quote::quote;
use syn::{
    Expr, FnArg, Ident, ItemFn, Lit, Pat, PatIdent, PatType, ReturnType, punctuated::Punctuated,
    token::Comma,
};

pub fn generate(input: &syn::ItemFn, output_ident: &Ident) -> proc_macro2::TokenStream {
    let name = input.sig.ident.to_string();
    let description = extract_description(input);
    let deprecated = extract_deprecated(input);
    let params = extract_params(input);
    let result = extract_result(input, output_ident);

    quote! {
        fn spec(&self) -> ::openspec_jsonrpsee::spec::Method {
            ::openspec_jsonrpsee::spec::Method {
                name: #name.into(),
                tags: None,
                summary: None,
                description: #description,
                external_docs: None,
                params: vec![#(#params),*],
                result: #result,
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
    let doc_lines = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                attr.meta.require_name_value().ok().map(|v| match &v.value {
                    Expr::Lit(e) => Some(e.lit.clone()),
                    _ => None,
                })
            } else {
                None
            }
        })
        .filter_map(|lit| {
            if let Some(Lit::Str(lit_str)) = lit {
                Some(lit_str.value().trim_start().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    if doc_lines.is_empty() {
        quote! { None }
    } else {
        quote! { Some(String::from(#doc_lines)) }
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
                ::openspec_jsonrpsee::spec::ContentDescriptor {
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

/// Generate the result spec component of the function
fn extract_result(input: &ItemFn, output_ident: &Ident) -> proc_macro2::TokenStream {
    let name = format!("{output_ident}Response");
    let schema = match &input.sig.output {
        ReturnType::Default => quote! { schemars::schema_for!(()) },
        ReturnType::Type(_, ty) => quote! { schemars::schema_for!(#ty) },
    };
    let is_deprecated = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("deprecated"));

    quote! {
        Some(::openspec_jsonrpsee::spec::ContentDescriptor {
            name: String::from(#name),
            summary: None,
            description: None,
            required: Some(true),
            schema: #schema,
            deprecated: Some(#is_deprecated),
        })
    }
}
