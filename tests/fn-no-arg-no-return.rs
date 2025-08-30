#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use openspec_jsonrpsee::{SpecModule, rpc};

#[rpc]
/// This is a doc comment for the do_something method.
fn do_something() {
    println!("hello world");
}

#[test]
pub fn test_jsonrpsee() {
    let mut module = SpecModule::new(());
    module
        .add_method(DoSomething)
        .expect("proof of concept should be able to register");
}
