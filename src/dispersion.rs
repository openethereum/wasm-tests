#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::Vec;

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (input, result) = unsafe { pwasm_std::parse_args(desc) };

    let mut dispersed = Vec::with_capacity(input.as_ref().len() * 2);
    for e in input.as_ref() {
        dispersed.push(*e);
        dispersed.push(e % 19);
    }

    result.done(dispersed);
}