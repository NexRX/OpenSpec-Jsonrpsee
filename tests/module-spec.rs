#![allow(deprecated)]

use openspec_jsonrpsee::{SpecModule, rpc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct StructA {
    name: String,
    b: StructB,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
struct StructB {
    name: String,
}

#[rpc]
/// This is a method description
fn method_a(value: StructA) -> StructB {
    value.b
}

#[rpc]
#[deprecated]
fn method_b(value: StructB) -> String {
    value.name
}

#[tokio::test]
async fn test_generated_spec() -> Result<(), Box<dyn std::error::Error>> {
    let mut module = SpecModule::new(());
    let spec = module.add_method(MethodA)?.add_method(MethodB)?.spec();

    assert_eq!(spec.methods.len(), 2);
    assert_eq!(spec.info.title, env!("CARGO_PKG_NAME"));
    assert_eq!(spec.info.version, env!("CARGO_PKG_VERSION"));

    let a = spec.methods[0].clone();
    assert_eq!(a.name, "method_a");
    assert_eq!(a.deprecated, Some(false));
    assert_eq!(a.description, Some("This is a method description".into()));
    assert!(a.servers.is_none());
    assert!(a.errors.is_none());
    assert_eq!(a.params.len(), 1);
    assert_eq!(a.params[0].name, "value");
    assert_eq!(a.params[0].description, None);
    assert_eq!(a.params[0].required, Some(true));
    assert_eq!(a.params[0].schema, schemars::schema_for!(StructA));
    let a_result = a.result.clone().expect("method a should have spec result");
    assert_eq!(a_result.schema, schemars::schema_for!(StructB));

    let b = spec.methods[1].clone();
    assert_eq!(b.name, "method_b");
    assert_eq!(b.deprecated, Some(true));
    assert_eq!(b.description, None);
    assert!(b.servers.is_none());
    assert!(b.errors.is_none());
    assert_eq!(b.params.len(), 1);
    assert_eq!(b.params[0].name, "value");
    assert_eq!(b.params[0].description, None);
    assert_eq!(b.params[0].required, Some(true));
    assert_eq!(b.params[0].schema, schemars::schema_for!(StructB));
    let b_result = b.result.clone().expect("method b should have spec result");
    assert_eq!(b_result.schema, schemars::schema_for!(String));

    Ok(())
}
