use crate::EasyModule;
use jsonrpsee::{http_client::HttpClient, server::Server};
use std::net::SocketAddr;

pub async fn test_server<Context: Send + Sync + 'static>(
    module: EasyModule<Context>,
) -> anyhow::Result<(HttpClient, SocketAddr)> {
    let server = Server::builder()
        .build("127.0.0.1:0".parse::<SocketAddr>()?)
        .await?;

    let module = module.into_jsonrpsee_module();

    let addr = server.local_addr()?;
    let handle = server.start(module);
    assert!(!handle.is_stopped());

    let client: HttpClient = jsonrpsee::http_client::HttpClientBuilder::default()
        .build(format!("http://{addr}"))
        .expect("client should be created");

    tokio::spawn(handle.stopped());

    Ok((client, addr))
}
