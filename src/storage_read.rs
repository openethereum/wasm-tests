#![no_main]

mod helpers;

use helpers::{CallArgs, storage, write_u32};

fn get_value_from_key(key: u32, val: &mut [u8; 32]) {
    let mut val = val;
    let mut full_key = [0u8; 32];
    write_u32(&mut full_key[0..4], key);
    let _ = storage::read(&full_key, &mut val);
}

#[no_mangle]
pub fn call(descriptor: *mut u8) {
	let mut ctx = unsafe { CallArgs::from_raw(descriptor) };
    let mut val = [0u8; 32];
	get_value_from_key(1, &mut val);

    let vec = val.to_vec();

    *ctx.result_mut() = vec;

	unsafe { ctx.save(descriptor); }
}