#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use easy_rpc::{EasyModule, Method, SyncCallback, test::test_server};
use easy_rpc_macros::rpc;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[rpc]
/// This is a doc comment for the method.
pub fn simple_types(first_arg: String) -> String {
    format!("Response: {first_arg}")
}

#[tokio::test]
async fn test_handles_simple_types() {
    let mut module = EasyModule::new(());
    module
        .add_method(SimpleTypes)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = SimpleTypes::request_unchecked(&client, "hello".into()).await;
    assert_eq!("Response: hello".to_string(), response)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct User {
    id: u32,
    name: String,
}

#[rpc]
/// This is a doc comment for the method.
pub fn struct_types(mut user: User) -> User {
    user.name = format!("{}-{}", user.id, user.name);
    user
}

#[tokio::test]
async fn test_handles_struct_types() {
    let mut module = EasyModule::new(());
    module
        .add_method(SimpleTypes)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = SimpleTypes::request_unchecked(&client, "hello".into()).await;
    assert_eq!("Response: hello".to_string(), response)
}
