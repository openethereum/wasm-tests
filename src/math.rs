#![no_main]
#![no_std]

extern crate wasm_std;

use wasm_std::{CallArgs, bigint};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let result = {
        let code = ctx.params().args()[0];

        let a_param: bigint::U256 = (&ctx.params().args()[1..33]).into();
        let b_param: bigint::U256 = (&ctx.params().args()[33..65]).into();

        let result = match code {
            0 => { a_param + b_param },
            1 => { a_param * b_param },
            2 => { a_param - b_param },
            _ => { a_param / b_param },
        };

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        result_bytes
    };

    *ctx.result_mut() = result.to_vec().into_boxed_slice();

    unsafe { ctx.save(desc); }
}