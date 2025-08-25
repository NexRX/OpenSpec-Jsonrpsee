#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use easy_rpc::{EasyModule, Method, SyncCallback, spec::ContentDescriptor};
use jsonrpsee::{
    Extensions, RpcModule,
    core::RpcResult,
    types::{ErrorObjectOwned, Params},
};

pub fn do_something() {
    println!("hello world");
}

fn callback() -> SyncCallback<(), RpcResult<()>> {
    fn do_something_rpc_wrapper<'a, 'b, 'c>(
        params: Params<'a>,
        _context: &'b (),
        _ext: &'c Extensions,
    ) -> RpcResult<()> {
        let (a, b): (String, String) = params.parse()?;
        do_something();
        Ok(())
    }

    do_something_rpc_wrapper
}

#[test]
pub fn test_jsonrpsee() {
    let mut module = RpcModule::new(());
    module
        .register_method("method.name()", callback())
        .expect("proof of concept should be able to register");
}
