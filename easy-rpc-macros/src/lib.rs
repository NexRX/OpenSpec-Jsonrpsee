pub(crate) mod rpc;
pub(crate) mod spec;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rpc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    rpc::generate_rpc_method(input)
}
