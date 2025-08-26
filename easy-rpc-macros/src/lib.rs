pub(crate) mod helpers;
pub(crate) mod rpc_method;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_attribute]
pub fn rpc(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = rpc_method::RpcMethodArgs::parse(args);
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    rpc_method::generate_rpc_method(input, args)
}
