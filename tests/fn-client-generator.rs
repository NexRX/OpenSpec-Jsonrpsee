#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use jsonrpsee::http_client::HttpClient;
use openspec_jsonrpsee::{SpecModule, rpc, test::test_server};

struct MyClient {
    client: HttpClient,
}

#[rpc(client = MyClient)]
/// This is a doc comment for the method.
fn simple_method() -> String {
    "Hello, World!".to_string()
}

#[rpc(client = MyClient)]
/// This is a doc comment for the method.
fn different_method() -> String {
    "Hello, Different World!".to_string()
}

#[tokio::test]
async fn test_handles_simple_args() {
    let mut module = SpecModule::new(());
    module.add_method(SimpleMethod).unwrap();
    module.add_method(DifferentMethod).unwrap();

    let (client, _addr) = test_server(module).await.expect("server should start");
    let client = MyClient { client };

    let response = client.simple_method().await.expect("should work");
    assert_eq!(response, "Hello, World!");

    let response = client.different_method().await.expect("should work");
    assert_eq!(response, "Hello, Different World!");
}

struct UnitClient(HttpClient);

#[rpc(client = UnitClient, client_field = 0)]
/// This is a doc comment for the method.
fn another_method() -> String {
    "Hello, Another World!".to_string()
}

#[tokio::test]
async fn test_unit_client() {
    let mut module = SpecModule::new(());
    module.add_method(AnotherMethod).unwrap();

    let (client, _addr) = test_server(module).await.expect("server should start");
    let res = UnitClient(client).another_method().await.unwrap();
    assert_eq!(res, "Hello, Another World!");
}
