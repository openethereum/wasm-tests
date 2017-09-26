#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::ext;

#[no_mangle]
pub fn call(desc: *mut u8) {
    unsafe { pwasm_std::parse_args(desc) }.1.done(ext::sender().to_vec());
}