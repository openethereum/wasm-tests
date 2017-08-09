#![no_main]
#![no_std]

extern crate wasm_std;

use wasm_std::{CallArgs, Vec};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let mut data = Vec::with_capacity(1);
    data.push(0u8);
    for arg in ctx.params().args() {
        // NOTE: reallocation happens here. Causes runtime "index out of bounds" if external malloc used
        // Thus we do not apply wasm_utils::externalize of "_free", "_malloc" here
        data.push(*arg);
    }

    *ctx.result_mut() = data.into_boxed_slice();

    unsafe { ctx.save(desc); }
}