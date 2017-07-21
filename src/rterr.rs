#![no_main]

mod helpers;

use helpers::logger;

#[no_mangle]
pub fn call(_desc: *mut u8) {
    let vc = vec![0u8; 5];
    logger::debug(&format!("Exception will occur here {}", vc[5]));
}