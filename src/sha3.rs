#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{CallArgs, sha3};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };
    let res = sha3("");
    *ctx.result_mut() = res.0.to_vec().into_boxed_slice();
    unsafe { ctx.save(desc); }
}
