#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use easy_rpc::{EasyModule, Method, SyncCallback, test::run_server};
use easy_rpc_macros::rpc;
use jsonrpsee::{
    Extensions,
    core::{client::ClientT, params::ArrayParams},
    rpc_params,
    types::Params,
};
use serde_json::json;

#[rpc]
/// This is a doc comment for the do_something method.
fn do_something(first_arg: String) -> String {
    format!("Response: {first_arg}")
}

#[tokio::test]
async fn test_jsonrpsee() {
    let mut module = EasyModule::new(());
    module
        .add_method(do_something)
        .expect("proof of concept should be able to register");

    let (addr, _) = run_server(module).await.expect("server should start");

    let client = jsonrpsee::http_client::HttpClientBuilder::default()
        .build(format!("http://{addr}"))
        .expect("client should be created");

    let params = rpc_params!("hello");

    let response: String = client
        .request::<String, _>("do_something", params)
        .await
        .expect("call should succeed");

    assert_eq!("Response: hello", response);
}
