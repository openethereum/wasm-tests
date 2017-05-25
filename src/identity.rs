#![feature(link_args)]
#![no_main]

mod helpers;

use helpers::CallArgs;

#[no_mangle]
pub fn call(desc: *mut u8) {   
    let mut ctx = CallArgs::from_raw(desc);

    let sender = ctx.params().sender().to_vec();

    *ctx.result_mut() = sender.to_vec();

    ctx.save(desc)
}