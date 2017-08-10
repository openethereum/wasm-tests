#![no_main]
#![no_std]
#![allow(deprecated)]

#[macro_use] extern crate wasm_std;

use wasm_std::{CallArgs, ext, write_u32, logger};
use core::hash::{SipHasher, Hasher};

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let addr = [13u8, 19, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let input = [1u8, 2, 3, 5, 7, 11];
    let mut result = vec![0u8; 256];
    match ext::call_code(&addr, &input, &mut result[..]) {
        Ok(_) => {
            logger::debug("Call succeed");
        },
        Err(_) => {
            logger::debug("Call failed");
        }
    }

    let mut hasher = SipHasher::new_with_keys(127, 129);
    hasher.write(&result[..]);
    let hash = (hasher.finish() & 0x00000000ffffffff) as u32;
    logger::debug("Hashing succed");

    let mut result = [0u8; 4];
    write_u32(&mut result[..], hash);
    *ctx.result_mut() = result.to_vec().into_boxed_slice();

    logger::debug("Exiting...");
    unsafe { ctx.save(desc); }
}