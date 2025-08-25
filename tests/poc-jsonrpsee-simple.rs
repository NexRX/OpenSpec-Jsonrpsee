#![allow(clippy::type_complexity)]
#![allow(clippy::unused_unit)]

use easy_rpc::{EasyModule, Method, SyncCallback};
use jsonrpsee::{Extensions, core::RpcResult, types::Params};

mod do_something_impl {
    // This would be annotated with #[rpc]
    pub fn do_something() {
        println!("hello world");
    }
}

#[allow(non_camel_case_types)]
pub struct do_something;

impl Method<(), ()> for do_something {
    fn name(&self) -> &'static str {
        "do_something"
    }

    fn spec(&self) -> easy_rpc::spec::Method {
        easy_rpc::spec::Method {
            name: "do_something".into(),
            tags: None,
            summary: None,
            description: None,
            external_docs: None,
            params: vec![],
            result: None,
            deprecated: Some(false),
            servers: None,
            errors: None,
            links: None,
            param_structure: None,
            examples: None,
        }
    }

    fn callback(&self) -> SyncCallback<(), RpcResult<()>> {
        fn do_something_rpc_wrapper<'a, 'b, 'c>(
            _params: Params<'a>,
            _context: &'b (),
            _ext: &'c Extensions,
        ) -> RpcResult<()> {
            do_something_impl::do_something();
            Ok(())
        }

        do_something_rpc_wrapper
    }
}

#[test]
pub fn test_jsonrpsee() {
    let mut module = EasyModule::new(());
    module
        .add_method(do_something)
        .expect("proof of concept should be able to register");
}
