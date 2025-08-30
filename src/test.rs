//! This module provides utilities for testing an `SpecModule` with a JSON-RPC server.
//! It includes a function to set up a test server and client for integration testing.
use crate::SpecModule;
use jsonrpsee::{http_client::HttpClient, server::Server};
use std::net::SocketAddr;

/// Sets up a test JSON-RPC server and client for the provided `SpecModule`.
///
/// # Type Parameters
/// - `Context`: The context type that must implement `Send` and `Sync` traits and have a static lifetime.
///
/// # Arguments
/// - `module`: The `SpecModule` to be tested, which will be converted into a JSON-RPC module.
///
/// # Returns
/// - `Ok((HttpClient, SocketAddr))`: A tuple containing the HTTP client and the server's socket address.
/// - `Err(std::io::error::Error)`: An error if the server setup fails.
///
/// # Example
/// ```no_run
/// use openspec_jsonrpsee::{SpecModule, test_server};
/// use jsonrpsee::server::Server;
/// use std::net::SocketAddr;
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let module = SpecModule::new(());
///     let (client, addr) = test_server(module).await?;
///     println!("Test server running at: {}", addr);
///     Ok(())
/// }
/// ```
pub async fn test_server<Context: Send + Sync + 'static>(
    module: SpecModule<Context>,
) -> std::io::Result<(HttpClient, SocketAddr)> {
    // Build a new JSON-RPC server bound to a random available port.
    let server = Server::builder()
        .build(
            "127.0.0.1:0"
                .parse::<SocketAddr>()
                .map_err(std::io::Error::other)?,
        )
        .await?;

    // Convert the provided `SpecModule` into a JSON-RPC module.
    let module = module.into_jsonrpsee_module();

    // Retrieve the local address of the server.
    let addr = server.local_addr()?;
    // Start the server with the provided module.
    let handle = server.start(module);
    // Ensure the server handle indicates the server is running.
    assert!(!handle.is_stopped());

    // Create an HTTP client to interact with the test server.
    let client: HttpClient = jsonrpsee::http_client::HttpClientBuilder::default()
        .build(format!("http://{addr}"))
        .expect("client should be created");

    // Spawn a task to monitor the server's stopped state.
    tokio::spawn(handle.stopped());

    Ok((client, addr))
}
