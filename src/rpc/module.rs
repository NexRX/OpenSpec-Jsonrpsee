//! This module defines the `SpecModule` struct, which provides a simplified interface for
//! creating and managing JSON-RPC modules using the `jsonrpsee` library. It allows for
//! openspec_ registration of synchronous and asynchronous RPC methods, as well as conversion
//! into a `jsonrpsee::RpcModule`.
use std::path::Path;

use crate::{OpenRpcSpec, RpcMethod, ServerHandler, spec};
use jsonrpsee::core::RegisterMethodError;
use serde::Serialize;

/// A wrapper around `jsonrpsee::RpcModule` that simplifies the process of
/// registering RPC methods and managing their specifications.
///
/// # Example Usage
///
/// ```
/// use openspec_jsonrpsee::{SpecModule, rpc};
/// use jsonrpsee::server::Server;
///
/// #[rpc]
/// async fn async_do_something(_first_arg: String) -> String {
///     // Simulate some async work
///     let result = async { "hello async world".to_string() };
///     tokio::time::sleep(std::time::Duration::from_millis(2)).await;
///     result.await
/// }
///
///
/// pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
///     // Register the method(s) in the module
///     let mut module = SpecModule::new(());
///     module.add_method(AsyncDoSomething)?;
///
///     let server = Server::builder()
///         .build("127.0.0.1:8080".parse::<std::net::SocketAddr>()?)
///         .await?;
///
///     let module = module.into_jsonrpsee_module();
///
///     let handle = server.start(module);
///     Ok(())
/// }
/// ```
///
/// # Type Parameters
/// - `Context`: The context type that will be passed to all registered methods.
///   Defaults to `()` if not specified.
pub struct SpecModule<Context = ()> {
    /// The underlying `jsonrpsee::RpcModule` instance.
    module: jsonrpsee::RpcModule<Context>,
    /// OpenRPC Specification
    spec: OpenRpcSpec,
}

impl<Context: Send + Sync + 'static> SpecModule<Context> {
    /// Creates a new `SpecModule` with the given context.
    ///
    /// # Arguments
    /// - `context`: The context to be passed to all registered methods.
    ///
    /// # Returns
    /// A new instance of `SpecModule`.
    pub fn new(context: Context) -> Self {
        SpecModule {
            module: jsonrpsee::RpcModule::new(context),
            spec: OpenRpcSpec::builder().build(),
        }
    }

    /// Sets the OpenRPC Specification's info.
    pub fn set_spec_info(&mut self, info: spec::Info) {
        self.spec.info = info;
    }

    /// Sets the OpenRPC Specification's servers
    pub fn set_spec_servers(&mut self, servers: Vec<spec::Server>) {
        self.spec.servers = Some(servers);
    }

    /// Sets the OpenRPC Specification's external documentation
    pub fn set_spec_external_docs(&mut self, external_docs: spec::ExternalDocumentation) {
        self.spec.external_docs = Some(external_docs);
    }

    /// Returns a reference to the OpenRPC Specification (semver 2.0.0).
    pub fn spec(&self) -> &OpenRpcSpec {
        &self.spec
    }

    /// Writes the OpenRPC Specification with every info & method added so far to a file.
    pub fn write_spec(&self, filepath: &Path) -> Result<(), std::io::Error> {
        std::fs::write(filepath, self.spec.to_string_pretty())
    }

    /// Adds a new RPC method to the module.
    ///
    /// # Type Parameters
    /// - `T`: The type of the response produced by the method. Must implement
    ///   `Serialize`, `Clone`, and `'static`.
    ///
    /// # Arguments
    /// - `method`: An implementation of the `RpcMethod` trait that defines the
    ///   method's name, specification, and handler.
    ///
    /// # Returns
    /// - `Ok(())` if the method was successfully registered.
    /// - `Err(RegisterMethodError)` if there was an error registering the method.
    pub fn add_method<T: Serialize + Clone + 'static>(
        &mut self,
        method: impl RpcMethod<Context, T>,
    ) -> Result<&mut Self, RegisterMethodError> {
        self.spec.methods.push(method.spec());

        match method.handler() {
            ServerHandler::Sync(handler) => {
                self.module.register_method(method.name(), handler)?;
            }
            ServerHandler::Async(handler) => {
                self.module.register_async_method(method.name(), handler)?;
            }
        }

        Ok(self)
    }

    /// Consumes the `SpecModule` and converts it into a `jsonrpsee::RpcModule`.
    ///
    /// # Returns
    /// The underlying `jsonrpsee::RpcModule` instance.
    pub fn into_jsonrpsee_module(self) -> jsonrpsee::RpcModule<Context> {
        self.module
    }
}

impl<Context: Send + Sync + 'static> From<SpecModule<Context>> for jsonrpsee::RpcModule<Context> {
    fn from(val: SpecModule<Context>) -> Self {
        val.into_jsonrpsee_module()
    }
}
