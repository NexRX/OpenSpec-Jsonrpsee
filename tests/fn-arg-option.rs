#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use openspec_jsonrpsee::{SpecModule, rpc, test::test_server};
use serde_json::json;

#[rpc]
/// This is a doc comment for the method.
fn optional_arg(first_arg: Option<String>) -> String {
    format!("Response: {first_arg:?}")
}

#[tokio::test]
async fn test_handles_none() {
    let mut module = SpecModule::new(());
    module
        .add_method(OptionalArg)
        .expect("proof of concept should be able to register");

    let spec = module.spec().clone();
    let arg_type = spec.methods[0].params[0].schema.get("type");
    assert_eq!(arg_type, Some(&json!(["string", "null"])));
    assert_eq!(spec.methods[0].params[0].required, Some(true));

    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = OptionalArg::request_unchecked(&client, None).await;
    assert_eq!("Response: None".to_string(), response)
}
