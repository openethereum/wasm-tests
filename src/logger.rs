#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{CallArgs, storage, write_u32};
use pwasm_std::bigint::U256;
use pwasm_std::hash::H256;

fn set_key_from_addr(key: u32, val: &[u8]) {
	let mut full_key = [0u8; 32];
	let mut full_val = [0u8; 32];

	write_u32(&mut full_key[0..4], key);
	full_val[12..32].copy_from_slice(val);

	let _ = storage::write(&H256::from(full_key), &full_val);
}

fn set_key_from_u256(key: u32, val: U256) {
	let mut full_key = [0u8; 32];
	write_u32(&mut full_key[0..4], key);

	let _ = storage::write(&H256::from(full_key), &val.into());
}

#[no_mangle]
pub fn call(descriptor: *mut u8) {
	// This initializes safe wrapper for contract input and output
	let ctx = unsafe { CallArgs::from_raw(descriptor) };

	set_key_from_addr(1, &ctx.params().address());
	set_key_from_addr(2, &ctx.params().sender());
	set_key_from_addr(3, &ctx.params().origin());
	set_key_from_u256(4, ctx.params().value());

	// Saves the wrapper state to commit return stream
	unsafe { ctx.save(descriptor); }
}
