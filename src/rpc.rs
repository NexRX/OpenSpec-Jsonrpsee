use crate::spec;
use jsonrpsee::{
    Extensions, RpcModule,
    core::{RegisterMethodError, RpcResult},
    types::Params,
};
use serde::Serialize;

pub type SyncCallback<Context, Response> = fn(Params, &Context, &Extensions) -> Response;
pub type AsyncCallback<Context, Response> =
    fn(
        ::jsonrpsee::types::Params<'static>,
        ::std::sync::Arc<Context>,
        ::jsonrpsee::Extensions,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>;

pub enum ServerHandler<Context, Response> {
    Sync(SyncCallback<Context, Response>),
    Async(AsyncCallback<Context, Response>),
}

pub trait RpcMethod<Context, Response: Serialize + Clone + 'static = ()> {
    /// Returns the name of the method
    fn name(&self) -> &'static str;
    /// Returns a OpenRPC specification for the method
    fn spec(&self) -> spec::Method;
    /// Returns a function (static) that handles the RPC request for the server
    fn handler(&self) -> ServerHandler<Context, RpcResult<Response>>;
}

pub struct EasyModule<Context = ()> {
    module: RpcModule<Context>,
    methods: Vec<spec::Method>,
}

impl<Context: Send + Sync + 'static> EasyModule<Context> {
    pub fn new(context: Context) -> Self {
        EasyModule {
            module: RpcModule::new(context),
            methods: Vec::new(),
        }
    }

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

    pub fn into_jsonrpsee_module(self) -> RpcModule<Context> {
        self.module
    }
}

impl<Context: Send + Sync + 'static> From<EasyModule<Context>> for RpcModule<Context> {
    fn from(val: EasyModule<Context>) -> Self {
        val.into_jsonrpsee_module()
    }
}
