#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use easy_rpc::{EasyModule, rpc, test::test_server};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[rpc]
/// This is a doc comment for the method.
fn simple_args(first_arg: String) -> String {
    format!("Response: {first_arg}")
}

#[tokio::test]
async fn test_handles_simple_args() {
    let mut module = EasyModule::new(());
    module
        .add_method(SimpleArgs)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = SimpleArgs::request_unchecked(&client, "hello".into()).await;
    assert_eq!("Response: hello".to_string(), response)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct User {
    id: u32,
    name: String,
}

#[rpc]
/// This is a doc comment for the method.
pub fn struct_args(mut user: User) -> User {
    user.name = format!("{}-{}", user.id, user.name);
    user
}

#[tokio::test]
async fn test_handles_struct_args() {
    let mut module = EasyModule::new(());
    module
        .add_method(StructArgs)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");

    let user = User {
        id: 123,
        name: "John".to_string(),
    };
    let response = StructArgs::request_unchecked(&client, user.clone()).await;
    assert_ne!(user, response);
    assert_eq!(user.id, response.id);
    assert_eq!("123-John", response.name);
}

/// Test multiple types

#[rpc]
/// This is a doc comment for the method.
fn multiple_args(mut user: User, prefix: String) -> User {
    user.name = format!("{}-{}", user.name, prefix);
    user
}

#[tokio::test]
async fn test_handles_multiple_args() {
    let mut module = EasyModule::new(());
    module
        .add_method(MultipleArgs)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");

    let mut user = User {
        id: 123,
        name: "John".to_string(),
    };
    let prefix = "prefix".to_string();
    let response = MultipleArgs::request_unchecked(&client, user.clone(), prefix.clone()).await;

    user.name = format!("{}-{}", user.name, prefix);
    assert_eq!(user, response);
}

/// Test struct in struct

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct StructA {
    name: String,
    b: StructB,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct StructB {
    name: String,
}

#[rpc]
/// This is a doc comment for the method.
fn struct_in_struct_args(value: StructA) -> StructB {
    value.b
}

#[tokio::test]
async fn test_handles_struct_in_struct() {
    let mut module = EasyModule::new(());
    module
        .add_method(StructInStructArgs)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");

    let value = StructA {
        name: "a".to_string(),
        b: StructB {
            name: "b".to_string(),
        },
    };
    let response = StructInStructArgs::request_unchecked(&client, value.clone()).await;
    assert_eq!(value.b, response);
}

/// Test References

#[rpc]
/// This is a doc comment for the method.
fn reference_args(value: &str) -> String {
    format!("Reference: {value}")
}

#[tokio::test]
async fn test_handles_reference_args() {
    let mut module = EasyModule::new(());
    module
        .add_method(ReferenceArgs)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");

    let response = ReferenceArgs::request_unchecked(&client, "hello").await;
    assert_eq!(response, "Reference: hello");
}

/// Test argument vector

#[rpc]
/// This is a doc comment for the method.
fn vector_args(values: Vec<String>) -> usize {
    values.len()
}

#[tokio::test]
async fn test_handles_vector_args() {
    let mut module = EasyModule::new(());
    module
        .add_method(VectorArgs)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");

    let values = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let response = VectorArgs::request_unchecked(&client, values.clone()).await;
    assert_eq!(response, values.len());
}
