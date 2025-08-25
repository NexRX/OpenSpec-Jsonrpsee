pub fn extract_return_type<'a>(input: &'a syn::ItemFn) -> syn::Type {
    match &input.sig.output {
        syn::ReturnType::Type(_, ty) => *ty.to_owned(),
        syn::ReturnType::Default => syn::parse_quote!(()),
    }
}

// Turns a &T into a T
pub fn remove_type_ref(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Reference(ref_type) => (*ref_type.elem).clone(),
        _ => ty.clone(),
    }
}

/// Converts a type like `str` to its natural owned version, e.g., `str` -> `String`.
pub fn owned_type_version(ty: &syn::Type) -> syn::Type {
    let ty = remove_type_ref(ty);
    match &ty {
        syn::Type::Path(type_path) => {
            // Check for bare `str`
            if type_path.qself.is_none()
                && type_path.path.segments.len() == 1
                && type_path.path.segments[0].ident == "str"
            {
                syn::parse_quote!(String)
            }
            // Check for bare `char`
            else if type_path.qself.is_none()
                && type_path.path.segments.len() == 1
                && type_path.path.segments[0].ident == "char"
            {
                // char is already owned
                ty.clone()
            }
            // Check for slices like `[T]` -> `Vec<T>`
            else if type_path.qself.is_none()
                && type_path.path.segments.len() == 1
                && type_path.path.segments[0].ident == ""
            {
                // This case is rare, but for completeness
                ty.clone()
            } else {
                ty.clone()
            }
        }
        syn::Type::Reference(ref_type) => {
            // If it's a reference to str, convert to String
            if let syn::Type::Path(type_path) = &*ref_type.elem {
                if type_path.qself.is_none()
                    && type_path.path.segments.len() == 1
                    && type_path.path.segments[0].ident == "str"
                {
                    syn::parse_quote!(String)
                } else if type_path.qself.is_none()
                    && type_path.path.segments.len() == 1
                    && type_path.path.segments[0].ident == "char"
                {
                    // char is already owned
                    syn::parse_quote!(char)
                } else {
                    (*ref_type.elem).clone()
                }
            } else {
                (*ref_type.elem).clone()
            }
        }
        syn::Type::Slice(slice_type) => {
            // &[T] or [T] -> Vec<T>
            let elem = &slice_type.elem;
            syn::parse_quote!(Vec<#elem>)
        }
        _ => ty.clone(),
    }
}
