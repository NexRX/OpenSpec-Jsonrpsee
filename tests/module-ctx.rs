#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use openspec_jsonrpsee::{SpecModule, rpc, test::test_server};

#[rpc]
/// This is a doc comment for the method.
fn unit_ctx(#[context] ctx: &()) -> String {
    format!("Response with unit context {ctx:?}")
}

#[tokio::test]
async fn test_unit_ctx() {
    let mut module = SpecModule::new(());
    module
        .add_method(UnitCtx)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = UnitCtx::request_unchecked(&client).await;

    assert_eq!("Response with unit context ()".to_string(), response);
}

#[rpc]
/// This is a doc comment for the method.
fn simple_ctx(#[context] ctx: &str) -> String {
    format!("Response with unit context {ctx:?}")
}

#[tokio::test]
async fn test_simple_ctx() {
    let mut module = SpecModule::new("string ctx".to_string());
    module
        .add_method(SimpleCtx)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = SimpleCtx::request_unchecked(&client).await;

    assert_eq!(
        "Response with unit context \"string ctx\"".to_string(),
        response
    );
}

struct CtxStruct {
    ctx_value: String,
}

#[rpc]
/// This is a doc comment for the method.
fn struct_ctx(#[context] ctx: &CtxStruct) -> String {
    format!("Response with unit context {:?}", ctx.ctx_value)
}

#[tokio::test]
async fn test_struct_ctx() {
    let mut module = SpecModule::new(CtxStruct {
        ctx_value: "string ctx".to_string(),
    });
    module
        .add_method(StructCtx)
        .expect("proof of concept should be able to register");

    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = StructCtx::request_unchecked(&client).await;

    assert_eq!(
        "Response with unit context \"string ctx\"".to_string(),
        response
    );
}
