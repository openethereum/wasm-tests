#![no_main]

mod helpers;

use helpers::{CallArgs, ext, logger};

#[no_mangle]
pub fn call(desc: *mut u8) {   
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    if let Ok(addr) = ext::create(&ctx.params().value(), ctx.params().args()) {
        logger::debug("Created contractwith code");
        *ctx.result_mut() = addr.to_vec();
    } else {
        logger::debug("Error creating contract");
    }

    unsafe { ctx.save(desc); }
}