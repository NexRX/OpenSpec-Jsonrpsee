//! This module defines the core abstractions for RPC methods and their handlers in the `easy_rpc` framework.
//!
//! The module provides:
//! - Type definitions for synchronous and asynchronous RPC callbacks.
//! - The `ServerHandler` enum to represent server-side handlers for RPC methods.
//! - The `RpcMethod` trait, which serves as the foundation for defining and registering RPC methods.
//!
//! The abstractions in this module are designed to work seamlessly with the `easy_rpc` framework,
//! enabling developers to define and manage RPC methods with minimal boilerplate.
//!
//! # Example
//! ```
//! use easy_rpc::{RpcMethod, ServerHandler};
//! use jsonrpsee::core::RpcResult;
//!
//! struct MyRpcMethod;
//!
//! impl RpcMethod<(), String> for MyRpcMethod {
//!     fn name(&self) -> &'static str {
//!         "my_rpc_method"
//!     }
//!
//!     fn spec(&self) -> easy_rpc::spec::Method {
//!         // Return the OpenRPC specification for the method
//!         unimplemented!()
//!     }
//!
//!     fn handler(&self) -> ServerHandler<(), RpcResult<String>> {
//!         // Return the server-side handler for the method
//!         unimplemented!()
//!     }
//! }
//! ```
use crate::spec;
use jsonrpsee::{Extensions, core::RpcResult, types::Params};
use serde::Serialize;

/// A synchronous callback for an RPC method.
pub type SyncCallback<Context, Response> = fn(Params, &Context, &Extensions) -> Response;
/// An asynchronous callback for an RPC method.
pub type AsyncCallback<Context, Response> =
    fn(
        ::jsonrpsee::types::Params<'static>,
        ::std::sync::Arc<Context>,
        ::jsonrpsee::Extensions,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>;

/// This enum represents the server-side handler for an RPC method, which can be either synchronous or asynchronous.
///
/// # Type Parameters
/// - `Context`: The type of the context can be passed to the handler.
/// - `Response`: The type of the response returned by the handler. Must implement [`Serialize`], [`Clone`], and `'static`.
pub enum ServerHandler<Context, Response> {
    Sync(SyncCallback<Context, Response>),
    Async(AsyncCallback<Context, Response>),
}

/// Represents an RPC method that can be registered with a [`easy_rpc::EasyModule`]].
/// You **aren't** expected to implement this trait directly. Instead, use the provided
/// [`easy_rpc::rpc`] macro to define your RPC methods.
///
/// # Type Parameters
/// - `Context`: The type of the context can be passed to the handler.
/// - `Response`: The type of the response returned by the handler. Must implement [`Serialize`], [`Clone`], and `'static`.
pub trait RpcMethod<Context, Response: Serialize + Clone + 'static = ()> {
    /// Returns the name of the RPC method.
    ///
    /// This name is used to identify the method in the RPC interface.
    fn name(&self) -> &'static str;

    /// Returns the OpenRPC specification for the method.
    ///
    /// The specification describes the method's parameters, result, and other metadata
    /// in accordance with the OpenRPC standard.
    fn spec(&self) -> spec::Method;

    /// Returns a function (static) that handles the RPC request for the server.
    ///
    /// The handler is responsible for processing incoming requests and producing a response.
    /// It can be either synchronous or asynchronous, as represented by [`ServerHandler`].
    fn handler(&self) -> ServerHandler<Context, RpcResult<Response>>;
}
