#![no_main]
#![no_std]

extern crate wasm_std;

use wasm_std::{CallArgs, Vec};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let mut dispersed = Vec::with_capacity(ctx.params().args().len() * 2);
    for e in ctx.params().args() {
        dispersed.push(*e);
        dispersed.push(e % 19);
    }

    *ctx.result_mut() = dispersed.into_boxed_slice();

    unsafe { ctx.save(desc); }
}