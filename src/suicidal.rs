#![feature(link_args)]
#![no_main]

mod helpers;

use helpers::{CallArgs, ext};

#[no_mangle]
pub fn call(desc: *mut u8) {   
    let mut ctx = CallArgs::from_raw(desc);

    if ctx.params().args().len() == 1 && ctx.params().args()[0] == 127 {
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&ctx.params().args()[1..]);
        ext::suicide(&addr);
    } else {
        *ctx.result_mut() = ctx.params().args().to_vec();
        ctx.save(desc);
    }
}