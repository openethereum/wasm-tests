#![no_main]
#![no_std]

extern crate wasm_std;

use wasm_std::CallArgs;

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let sender = ctx.params().sender().to_vec();

    *ctx.result_mut() = sender.to_vec().into_boxed_slice();

    unsafe { ctx.save(desc); }
}