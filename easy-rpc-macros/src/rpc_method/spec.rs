use quote::quote;

pub fn generate(input: &syn::ItemFn) -> proc_macro2::TokenStream {
    let name = &input.sig.ident;
    let description = gen_description(input);
    let deprecated = gen_deprecated(input);
    let params = gen_params(input);

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

fn gen_deprecated(input: &syn::ItemFn) -> proc_macro2::TokenStream {
    let is_deprecated = input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("deprecated"));

    quote! { Some(#is_deprecated) }
}

fn gen_description(input: &syn::ItemFn) -> proc_macro2::TokenStream {
    let doc_lines: Vec<String> = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                attr.parse_args::<syn::LitStr>().ok().map(|lit| lit.value())
            } else {
                None
            }
        })
        .collect();

    if doc_lines.is_empty() {
        quote! { None }
    } else {
        let doc = doc_lines.join("\n");
        quote! { Some(#doc) }
    }
}

fn gen_params(input: &syn::ItemFn) -> Vec<proc_macro2::TokenStream> {
    let mut params = Vec::new();

    for param in &input.sig.inputs {
        // Extract parameter name
        let name = match param {
            syn::FnArg::Typed(pat_type) => match &*pat_type.pat {
                syn::Pat::Ident(ident) => {
                    let ident = &ident.ident;
                    quote! { stringify!(#ident).into() }
                }
                _ => quote! { None },
            },
            syn::FnArg::Receiver(_) => quote! { "self".into() },
        };

        // For demonstration, summary and description are not extracted from param attributes here.
        let summary = quote! { None };
        let description = quote! { None };

        // Required: function parameters are always required
        let required = quote! { Some(true) };

        // Schema: use the type of the parameter
        let schema = match param {
            syn::FnArg::Typed(pat_type) => {
                let ty = &pat_type.ty;
                quote! { schemars::schema_for!(#ty) }
            }
            syn::FnArg::Receiver(_) => quote! { <panic> },
        };

        // Deprecated: check for #[deprecated] attribute on the param
        let deprecated = match param {
            syn::FnArg::Typed(pat_type) => {
                let is_deprecated = pat_type
                    .attrs
                    .iter()
                    .any(|attr| attr.path().is_ident("deprecated"));
                quote! { Some(#is_deprecated) }
            }
            syn::FnArg::Receiver(_) => quote! { None },
        };

        params.push(quote! {
            ::easy_rpc::spec::ContentDescriptor {
                name: #name,
                summary: #summary,
                description: #description,
                required: #required,
                schema: #schema,
                deprecated: #deprecated,
            }
        });
    }

    params
}
