use std::{net::SocketAddr, path::Path};

use jsonrpsee::server::Server;
use openspec_jsonrpsee::{SpecModule, rpc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub struct Ctx {
    pub database: SqlitePool,
}

#[derive(Clone, JsonSchema, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    id: i64,
    name: String,
    age: i64,
}

#[rpc]
pub async fn register_user(#[context] ctx: &Ctx, name: String, age: i64) -> Result<i64, String> {
    sqlx::query("INSERT INTO user (name, age) VALUES (?, ?)")
        .bind(name)
        .bind(age)
        .execute(&ctx.database)
        .await
        .map(|row| row.last_insert_rowid())
        .map_err(|err| err.to_string())
}

#[rpc]
pub async fn get_user(#[context] ctx: &Ctx, user_id: i64) -> Result<Option<User>, String> {
    sqlx::query_as::<_, User>("SELECT * FROM user WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&ctx.database)
        .await
        .map_err(|err| err.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const SERVER_ADDR: &str = "127.0.0.1:8080";
    let ctx: Ctx = Ctx {
        database: temp_database().await?,
    };

    let mut module = SpecModule::new(ctx);
    module
        .add_method(RegisterUser)?
        .add_method(GetUser)?
        .write_spec(Path::new("./spec.json"))?;

    let server = Server::builder()
        .build(
            SERVER_ADDR
                .parse::<SocketAddr>()
                .map_err(std::io::Error::other)?,
        )
        .await?;

    let module = module.into_jsonrpsee_module();
    let handle = server.start(module);

    println!("Server started at {SERVER_ADDR}");
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down.");
        }
        _ = handle.stopped() => {
            println!("Server stopped.");
        }
    }
    Ok(())
}

async fn temp_database() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    const SCHEMA: &str = "CREATE TABLE user (
          id INTEGER PRIMARY KEY,
          name TEXT NOT NULL,
          age INTEGER
      )";

    sqlx::query(SCHEMA).execute(&pool).await?;

    Ok(pool)
}
