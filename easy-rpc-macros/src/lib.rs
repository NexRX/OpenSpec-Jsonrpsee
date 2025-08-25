pub(crate) mod helpers;
pub(crate) mod rpc_method;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rpc(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    rpc_method::generate_rpc_method(input)
}
