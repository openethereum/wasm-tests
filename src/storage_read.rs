#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{CallArgs, storage, write_u32};
use pwasm_std::hash::H256;

fn get_value_from_key(key: u32) -> Result<[u8; 32], storage::Error> {
    let mut full_key = [0u8; 32];
    write_u32(&mut full_key[0..4], key);
    storage::read(&H256::from(full_key))
}

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(descriptor) };
    let val: [u8; 32] = match get_value_from_key(1) {
        Ok(v) => v,
        Err(_) => [0u8; 32]
    };

    *ctx.result_mut() = val.to_vec().into_boxed_slice();

    unsafe { ctx.save(descriptor); }
}
