mod handler;
mod request;
mod spec;

use crate::{helpers::extract_return_type, rpc_method::request::RequestImpl};
use heck::AsUpperCamelCase;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

pub fn generate_rpc_method(input: syn::ItemFn) -> TokenStream {
    let original_name = &input.sig.ident;
    let gen_name = Ident::new(
        &AsUpperCamelCase(original_name.to_string()).to_string(),
        original_name.span(),
    );
    let context_type = syn::parse_quote!(());

    let return_type = extract_return_type(&input);

    let fn_request = request::generate(&input, RequestImpl::Checked);
    let fn_request_unchecked = request::generate(&input, RequestImpl::Unchecked);

    let fn_name = gen_fn_name(original_name);
    let fn_spec = spec::generate(&input);
    let fn_handler = handler::generate(&input, &original_name, &context_type, &return_type);

    quote::quote! {
        #input

        #[allow(non_camel_case_types)]
        pub struct #gen_name;

        impl #gen_name {
            #fn_request
            #fn_request_unchecked
        }

        impl Method<(), #return_type> for #gen_name {
            #fn_name
            #fn_spec
            #fn_handler
        }
    }
    .into()
}

fn gen_fn_name(fn_name: &Ident) -> TokenStream2 {
    quote::quote! {
        fn name(&self) -> &'static str {
            stringify!(#fn_name)
        }
    }
}
