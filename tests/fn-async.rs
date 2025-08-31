#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use openspec_jsonrpsee::{SpecModule, rpc, test::test_server};

#[rpc]
/// This is a doc comment for the do_something method.
async fn async_do_something(_first_arg: String) -> String {
    // Simulate some async work
    let result = async { "hello async world".to_string() };
    tokio::time::sleep(std::time::Duration::from_millis(2)).await;
    result.await
}

#[tokio::test]
pub async fn test_loads() {
    let mut module = SpecModule::new(());
    module
        .add_method(AsyncDoSomething)
        .expect("proof of concept should be able to register");
    let (client, _addr) = test_server(module).await.expect("server should start");
    let response = AsyncDoSomething::request_unchecked(&client, "hello".into()).await;
    assert_eq!("hello async world".to_string(), response)
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

/// Fake user database to use for context
type UserDb = Arc<RwLock<HashMap<u32, User>>>;

#[derive(Clone, Debug, PartialEq, JsonSchema, Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    password: String,
}

#[rpc]
pub async fn get_user(#[context] ctx: UserDb, user_id: u32) -> Option<User> {
    ctx.read().await.get(&user_id).cloned()
}

#[rpc]
pub async fn register_user(
    name: String,
    password: String,
    #[context] ctx: UserDb,
) -> Result<User, String> {
    let mut db = ctx.write().await; // Acquire a write lock
    let id = db.len() as u32 + 1;
    let user = User { id, name, password };
    db.insert(id, user.clone());
    Ok(user)
}

/// Testing this compiles as not as simply to support as it looks
#[rpc]
pub async fn register_user_ref(
    name: String,
    password: String,
    #[context] ctx: &UserDb,
) -> Result<User, String> {
    let mut db = ctx.write().await; // Acquire a write lock
    let id = db.len() as u32 + 1;
    let user = User { id, name, password };
    db.insert(id, user.clone());
    Ok(user)
}

#[tokio::test]
pub async fn test_mutate_async() -> Result<(), Box<dyn std::error::Error>> {
    let users_ctx: UserDb = Arc::new(RwLock::new(HashMap::new()));
    let mut module = SpecModule::new(users_ctx);
    module
        .add_method(GetUser)?
        .add_method(RegisterUser)?
        .add_method(RegisterUserRef)?;

    let (client, _addr) = test_server(module).await.expect("server should start");

    // No users
    let response = GetUser::request_unchecked(&client, 1).await;
    assert_eq!(response, None);

    // First user inserts & reads
    let response = RegisterUser::request_unchecked(&client, "John".into(), "password".into()).await;
    assert!(response.is_ok());
    let response = GetUser::request_unchecked(&client, 1).await;
    assert_ne!(response, None);

    // Second user expands on state
    let response =
        RegisterUserRef::request_unchecked(&client, "Jane".into(), "password".into()).await;
    assert!(response.is_ok());
    let response = GetUser::request_unchecked(&client, 2).await;
    assert_ne!(response, None);

    Ok(())
}
