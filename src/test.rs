use crate::EasyModule;
use jsonrpsee::server::{Server, ServerHandle};
use std::net::SocketAddr;

pub async fn run_server<Context: Send + Sync + 'static>(
    module: EasyModule<Context>,
) -> anyhow::Result<(SocketAddr, ())> {
    let server = Server::builder()
        .build("127.0.0.1:0".parse::<SocketAddr>()?)
        .await?;

    let module = module.into_jsonrpsee_module();

    let addr = server.local_addr()?;
    let handle = server.start(module);
    assert!(!handle.is_stopped());

    tokio::spawn(handle.stopped());

    Ok((addr, ()))
}
