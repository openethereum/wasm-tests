#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{CallArgs, ext};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    if ctx.params().args().len() > 0 && ctx.params().args()[0] == 127 {
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&ctx.params().args()[1..]);
        ext::suicide(&addr);
    } else {
        *ctx.result_mut() = ctx.params().args().to_vec().into_boxed_slice();
        unsafe { ctx.save(desc); }
    }
}