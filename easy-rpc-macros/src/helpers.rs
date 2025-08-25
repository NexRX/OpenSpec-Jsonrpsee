pub fn extract_return_type<'a>(input: &'a syn::ItemFn) -> syn::Type {
    match &input.sig.output {
        syn::ReturnType::Type(_, ty) => *ty.to_owned(),
        syn::ReturnType::Default => syn::parse_quote!(()),
    }
}

/// Extract the argument with #[context]
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

/// Extract the argument attributed with #[context], returning a tuple of (as_owned_type, as_ref_type)
pub fn extract_pat_ty_or(
    pat_type: Option<&syn::PatType>,
    or_default: syn::Type,
) -> (syn::Type, syn::Type) {
    let ty = pat_type
        .clone()
        .map(|v| v.ty.as_ref().clone())
        .unwrap_or(or_default);

    // Remove the & from as_ref_type if it has one
    let as_ref_type = if let syn::Type::Reference(ref_type) = &ty {
        (*ref_type.elem).clone()
    } else {
        ty.clone()
    };

    let as_owned_type = if let syn::Type::Reference(ref_type) = &ty {
        match &*ref_type.elem {
            syn::Type::Path(type_path) => {
                // Check for &str and convert to String
                if type_path.qself.is_none()
                    && type_path.path.segments.len() == 1
                    && type_path.path.segments[0].ident == "str"
                {
                    syn::parse_quote!(String)
                } else {
                    (*ref_type.elem).clone()
                }
            }
            _ => (*ref_type.elem).clone(),
        }
    } else {
        ty.clone()
    };

    (as_owned_type, as_ref_type)
}
