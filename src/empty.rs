#![no_main]
#![no_std]

extern crate wasm_std;

use wasm_std::logger;

#[no_mangle]
pub fn call(_desc: *mut u8) {
    logger::debug("Empty contract");
}