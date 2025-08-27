//! This module defines the `EasyModule` struct, which provides a simplified interface for
//! creating and managing JSON-RPC modules using the `jsonrpsee` library. It allows for
//! openspec_ registration of synchronous and asynchronous RPC methods, as well as conversion
//! into a `jsonrpsee::RpcModule`.
use crate::{RpcMethod, ServerHandler, spec};
use jsonrpsee::core::RegisterMethodError;
use serde::Serialize;

/// A wrapper around `jsonrpsee::RpcModule` that simplifies the process of
/// registering RPC methods and managing their specifications.
///
/// # Example Usage
///
/// ```
/// use easy_rpc::{EasyModule, rpc};
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
///     let mut module = EasyModule::new(());
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
pub struct EasyModule<Context = ()> {
    /// The underlying `jsonrpsee::RpcModule` instance.
    module: jsonrpsee::RpcModule<Context>,
    /// A collection of method specifications for the registered RPC methods.
    methods: Vec<spec::Method>,
}

impl<Context: Send + Sync + 'static> EasyModule<Context> {
    /// Creates a new `EasyModule` with the given context.
    ///
    /// # Arguments
    /// - `context`: The context to be passed to all registered methods.
    ///
    /// # Returns
    /// A new instance of `EasyModule`.
    pub fn new(context: Context) -> Self {
        EasyModule {
            module: jsonrpsee::RpcModule::new(context),
            methods: Vec::new(),
        }
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
    ) -> Result<(), RegisterMethodError> {
        self.methods.push(method.spec());

        match method.handler() {
            ServerHandler::Sync(handler) => {
                self.module.register_method(method.name(), handler)?;
            }
            ServerHandler::Async(handler) => {
                self.module.register_async_method(method.name(), handler)?;
            }
        }

        Ok(())
    }

    /// Consumes the `EasyModule` and converts it into a `jsonrpsee::RpcModule`.
    ///
    /// # Returns
    /// The underlying `jsonrpsee::RpcModule` instance.
    pub fn into_jsonrpsee_module(self) -> jsonrpsee::RpcModule<Context> {
        self.module
    }
}

impl<Context: Send + Sync + 'static> From<EasyModule<Context>> for jsonrpsee::RpcModule<Context> {
    fn from(val: EasyModule<Context>) -> Self {
        val.into_jsonrpsee_module()
    }
}
