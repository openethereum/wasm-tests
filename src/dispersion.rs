#![feature(link_args)]
#![no_main]

mod helpers;

use helpers::CallArgs;

#[no_mangle]
pub fn call(desc: *mut u8) {   
    let mut ctx = CallArgs::from_raw(desc);

    let mut dispersed = Vec::with_capacity(ctx.params().args().len() * 2);
    for e in ctx.params().args() {
        dispersed.push(*e);
        dispersed.push(e % 19);
    }

    *ctx.result_mut() = dispersed;

    ctx.save(desc)
}