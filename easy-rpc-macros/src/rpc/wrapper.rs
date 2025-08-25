// Returns the generated wrapper module for the input function and a full reference to the function inside
pub(super) fn gen_fn_wrapper(
    input: &syn::ItemFn,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let name = &input.sig.ident;
    let name_impl = syn::Ident::new(&format!("{}_impl", name), name.span());
    let original_vis = &input.vis;
    let mut public_input = input.clone();
    public_input.vis = syn::Visibility::Public(syn::token::Pub::default());

    (
        quote::quote! {
            #original_vis mod #name_impl {
                #public_input
            }
        },
        quote::quote! { #name_impl::#name },
    )
}
