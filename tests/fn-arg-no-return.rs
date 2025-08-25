#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use easy_rpc::{EasyModule, Method, SyncCallback};
use easy_rpc_macros::rpc;
use jsonrpsee::{Extensions, types::Params};

#[rpc]
/// This is a doc comment for the do_something method.
fn do_something(first_arg: String) {
    println!("hello world");
}

#[test]
pub fn test_jsonrpsee() {
    let mut module = EasyModule::new(());
    module
        .add_method(do_something)
        .expect("proof of concept should be able to register");
}
