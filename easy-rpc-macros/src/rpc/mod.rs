mod callback;
mod wrapper;

use crate::{rpc::wrapper::gen_fn_wrapper, spec};
use proc_macro::TokenStream;

pub fn generate_rpc_method(input: syn::ItemFn) -> TokenStream {
    let (wrapper, wrapped_fn) = gen_fn_wrapper(&input);
    let name = &input.sig.ident;
    let spec_impl = spec::gen_method_impl(&input);
    let callback_impl = callback::gen_callback_impl(&input, &wrapped_fn);

    quote::quote! {
        #wrapper

        #[allow(non_camel_case_types)]
        pub struct #name;

        impl Method<(), ()> for #name {
            fn name(&self) -> &'static str {
                stringify!(#name)
            }

            #spec_impl

            #callback_impl
        }
    }
    .into()
}
