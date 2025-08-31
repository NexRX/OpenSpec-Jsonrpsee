
#[macro_use]
extern crate jsonrpc_client_core;

extern crate serde;
extern crate serde_json;
extern crate derive_builder;

use serde::{Serialize, Deserialize};
use derive_builder::Builder;
use std::collections::HashMap;
pub type IntegerXZTmW7Mv = i64;
pub type StringDoaGddGA = String;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ObjectOfIntegerXZTmW7MvDX83NGkO {
    #[serde(rename = "Ok")]
    pub ok: IntegerXZTmW7Mv,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ObjectOfStringDoaGddGARMaVIgow {
    #[serde(rename = "Err")]
    pub err: StringDoaGddGA,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9 {
    pub id: IntegerXZTmW7Mv,
    pub name: StringDoaGddGA,
    pub password: StringDoaGddGA,
}
pub type NullQu0Arl1F = serde_json::Value;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyOfNullQu0Arl1FObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9DbbVqH1Y {
    ObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9(ObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9),
    NullQu0Arl1F(NullQu0Arl1F),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct ObjectOfAnyOfNullQu0Arl1FObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9DbbVqH1Y01YbCDyu {
    #[serde(rename = "Ok")]
    pub ok: AnyOfNullQu0Arl1FObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9DbbVqH1Y,
}
pub type String = String;
pub type Int64 = i64;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Builder, Default)]
#[builder(setter(strip_option), default)]
#[serde(default)]
pub struct User {
    pub id: IntegerXZTmW7Mv,
    pub name: StringDoaGddGA,
    pub password: StringDoaGddGA,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResultOfInt64OrString {
    ObjectOfIntegerXZTmW7MvDX83NGkO(ObjectOfIntegerXZTmW7MvDX83NGkO),
    ObjectOfStringDoaGddGARMaVIgow(ObjectOfStringDoaGddGARMaVIgow),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ResultOfNullableUserOrString {
    ObjectOfAnyOfNullQu0Arl1FObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9DbbVqH1Y01YbCDyu(ObjectOfAnyOfNullQu0Arl1FObjectOfStringDoaGddGAStringDoaGddGAIntegerXZTmW7MvPDRL7Kw9DbbVqH1Y01YbCDyu),
    ObjectOfStringDoaGddGARMaVIgow(ObjectOfStringDoaGddGARMaVIgow),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnyOfStringInt64UserResultOfInt64OrStringResultOfNullableUserOrString {
    String(String),
    Int64(Int64),
    User(User),
    ResultOfInt64OrString(ResultOfInt64OrString),
    ResultOfNullableUserOrString(ResultOfNullableUserOrString),
}

jsonrpc_client!(pub struct OpenspecJsonrpsee {
  pub fn RegisterUser(&mut self, name: String, age: Int64) -> RpcRequest<ResultOfInt64OrString>;
pub fn GetUser(&mut self, user: User) -> RpcRequest<ResultOfNullableUserOrString>;
});
