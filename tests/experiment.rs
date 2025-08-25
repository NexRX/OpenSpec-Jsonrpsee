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
        let result = do_something();
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

#[test]
pub fn param_deserialize_one() {
    let params = Params::new(Some("[\"hello\"]"));
    let result = params.one::<String>();
    assert_eq!(Ok("hello".to_string()), result);
}

#[test]
pub fn param_deserialize_two() {
    let params = Params::new(Some("[\"hello\", 2]"));
    let result = params.parse::<(String, u32)>();
    assert_eq!(Ok(("hello".to_string(), 2)), result);
}
