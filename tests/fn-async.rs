#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use easy_rpc::{EasyModule, rpc, test::test_server};

#[rpc]
/// This is a doc comment for the do_something method.
async fn async_do_something(_first_arg: String) -> String {
    "hello async world".to_string()
}

#[tokio::test]
pub async fn test_jsonrpsee() {
    let mut module = EasyModule::new(());
    module
        .add_method_async(AsyncDoSomething)
        .expect("proof of concept should be able to register");
    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = AsyncDoSomething::request_unchecked(&client, "hello".into()).await;
    assert_eq!("hello async world".to_string(), response)
}
