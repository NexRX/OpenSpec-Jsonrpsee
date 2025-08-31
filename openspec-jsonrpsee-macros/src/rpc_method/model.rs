use crate::helpers::{extract_return_type, owned_type_version};
use heck::AsUpperCamelCase;
use proc_macro_error::abort;
use proc_macro2::Span;
use std::panic;
use syn::{
    punctuated::*,
    spanned::Spanned,
    token::{Async, Comma},
    *,
};

#[derive(Debug, darling::FromMeta)]
#[darling(derive_syn_parse)]
pub struct RpcMethodArgs {
    #[darling(default)]
    pub client: Option<syn::Path>,
    pub client_field: Option<Expr>,
}

impl RpcMethodArgs {
    pub fn parse(args: proc_macro::TokenStream) -> Self {
        match syn::parse::<Self>(args) {
            Ok(v) => v,
            Err(e) => abort!(e.span(), "Incorrect macro arguments: {:#?}", e),
        }
    }
}

#[derive(Clone)]
pub struct RpcMethod {
    pub input_span: Span,
    pub input_async: Option<Async>,
    pub input_ident: Ident,
    pub input_vis: Visibility,
    pub output_ident: Ident, // gen_name
    #[allow(dead_code)]
    /// True if input used #[context]
    pub context_needed: bool,
    pub context_ty_referenced: bool,
    #[allow(dead_code)]
    /// raw context type, e.g. &str
    pub context_ty: Type,
    // /// Non-reference version of context_ty, e.g. &str --> str  (use this to reference with lifetime etc)
    // pub context_ty_without_ref: Type,
    /// Owned version of context_ty, e.g. &str --> String
    pub context_ty_owned: Type,
    pub context_ident: Option<Ident>,
    #[allow(dead_code)]
    /// fn args e.g. `a: String, b: u32, c: Struct`
    pub fn_args: Punctuated<PatType, Comma>,
    /// fn args as idents E.g. `a, b, c`
    pub fn_args_as_ident: Punctuated<Ident, Comma>,
    /// fn args without context e.g. `a: String, b: u32, c: Struct`
    pub fn_args_contextless: Punctuated<PatType, Comma>,
    /// fn args without context as idents E.g. `a, b, c`
    pub fn_args_contextless_as_ident: Punctuated<Ident, Comma>,
    pub response_ty: Type,
}

impl RpcMethod {
    pub fn parse(input: ItemFn) -> Self {
        let context_ty = extract_context_arg(&input).map(|pat_type| pat_type.ty.as_ref().clone());
        let context_needed = context_ty.is_some();
        let context_ty = context_ty.unwrap_or_else(|| syn::parse_quote!(()));
        let fn_args = extract_fn_args(&input, false);
        let fn_args_contextless = extract_fn_args(&input, true);

        RpcMethod {
            input_async: input.sig.asyncness,
            input_span: input.span(),
            input_ident: input.sig.ident.clone(),
            input_vis: input.vis.clone(),
            output_ident: Ident::new(
                &AsUpperCamelCase(input.sig.ident.to_string()).to_string(),
                input.sig.ident.span(),
            ),
            context_needed,
            // context_ty_without_ref: remove_type_ref(&context_ty),
            context_ty_referenced: matches!(context_ty, syn::Type::Reference(_)),
            context_ty_owned: owned_type_version(&context_ty),
            context_ty,
            context_ident: extract_context_ident(&input),
            fn_args_as_ident: as_ident(&fn_args),
            fn_args,
            fn_args_contextless_as_ident: as_ident(&fn_args_contextless),
            fn_args_contextless,
            response_ty: extract_return_type(&input),
        }
    }
}

pub fn extract_context_arg(input: &syn::ItemFn) -> Option<&syn::PatType> {
    input.sig.inputs.iter().find_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            for attr in &pat_type.attrs {
                if attr.path().is_ident("context") {
                    return Some(pat_type);
                }
            }
        }
        None
    })
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

fn extract_fn_args(input: &syn::ItemFn, exclude_context: bool) -> Punctuated<syn::PatType, Comma> {
    input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => panic!("function cannot take self"),
            syn::FnArg::Typed(pat_type) => {
                // If exclude_context is true and this arg has #[context], skip it entirely
                if exclude_context
                    && pat_type
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident("context"))
                {
                    return None;
                }
                let mut pat_type = pat_type.clone();
                // Remove #[context] attribute if present
                pat_type
                    .attrs
                    .retain(|attr| !attr.path().is_ident("context"));
                // Remove `mut` for argument generation
                if let syn::Pat::Ident(pat_ident) = &mut *pat_type.pat {
                    pat_ident.mutability = None;
                }
                Some(pat_type)
            }
        })
        .collect()
}

pub fn as_ident(fn_args: &Punctuated<syn::PatType, Comma>) -> Punctuated<syn::Ident, Comma> {
    fn_args
        .iter()
        .filter_map(|arg| {
            if let syn::Pat::Ident(pat_ident) = arg.pat.as_ref() {
                Some(pat_ident.ident.clone())
            } else {
                panic!("Unexpected pattern type")
            }
        })
        .collect()
}
