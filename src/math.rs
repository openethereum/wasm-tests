#![no_main]
#![no_std]

extern crate wasm_std;

use wasm_std::{CallArgs, bigint};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let result = {
        let a_param: bigint::U256 = (&ctx.params().args()[0..32]).into();
        let b_param: bigint::U256 = (&ctx.params().args()[32..64]).into();

        let sum = a_param + b_param;
        let mut result = [0u8; 32];

        sum.to_big_endian(&mut result);

        result
    };

    *ctx.result_mut() = result.to_vec().into_boxed_slice();

    unsafe { ctx.save(desc); }
}