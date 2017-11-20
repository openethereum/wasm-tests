#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{storage, write_u32};
use pwasm_std::hash::H256;

fn get_value_from_key(key: u32) -> [u8; 32] {
    let mut full_key = [0u8; 32];
    write_u32(&mut full_key[0..4], key);
    storage::read(&H256::from(full_key))
}

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (_, result) = unsafe { pwasm_std::parse_args(desc) };
    let val: [u8; 32] = get_value_from_key(1);
    result.done(val.to_vec());
}
