#![no_main]

mod helpers;

use helpers::logger;

#[no_mangle]
pub fn call(_desc: *mut u8) {
    logger::debug("Empty contract");
}