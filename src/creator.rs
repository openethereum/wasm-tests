#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{CallArgs, ext, logger};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    if let Ok(addr) = ext::create(ctx.params().value(), ctx.params().args()) {
        logger::debug("Created contractwith code");
        *ctx.result_mut() = addr.to_vec().into_boxed_slice();
    } else {
        logger::debug("Error creating contract");
    }

    unsafe { ctx.save(desc); }
}
