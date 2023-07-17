#![allow(non_snake_case)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;

module_manifest!();

pub fn main() {}

#[marine]
pub fn greeting() -> bool {
    
    let res = action_1();
    action_2()
}


#[marine]
#[link(wasm_import_module = "ctrlr_1")]
extern "C" {
    pub fn action_1() -> bool;
}

#[marine]
#[link(wasm_import_module = "ctrlr_2")]
extern "C" {
    pub fn action_2() -> bool;
}
