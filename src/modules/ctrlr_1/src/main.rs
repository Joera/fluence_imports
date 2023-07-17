#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
pub fn action_1(name: String) -> bool {
    someFunction()
}

#[marine]
#[link(wasm_import_module = "shared_factory")]
extern "C" {
    pub fn someFunction() -> bool;
}