pub fn extract_return_type<'a>(input: &'a syn::ItemFn) -> syn::Type {
    match &input.sig.output {
        syn::ReturnType::Type(_, ty) => *ty.to_owned(),
        syn::ReturnType::Default => syn::parse_quote!(()),
    }
}
