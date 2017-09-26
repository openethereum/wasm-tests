#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::Vec;

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (input, result) = unsafe { pwasm_std::parse_args(desc) };

    result.done({
        let mut data = Vec::with_capacity(1);
        data.push(0u8);
        for arg in input.as_ref() {
            data.push(*arg);
        }
        data
    });
}