#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::keccak;

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (input, result) = unsafe { pwasm_std::parse_args(desc) };
    result.done(keccak(&input).0.to_vec());
}
