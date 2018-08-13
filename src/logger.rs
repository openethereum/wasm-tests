#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum as ext;
// extern crate bigint;
extern crate uint;

use pwasm_std::write_u32;
use pwasm_std::hash::H256;
// use bigint::U256;
use uint::U256;

fn set_key_from_addr(key: u32, val: &[u8]) {
	let mut full_key = [0u8; 32];
	let mut full_val = [0u8; 32];

	write_u32(&mut full_key[0..4], key);
	full_val[12..32].copy_from_slice(val);

	let _ = ext::write(&H256::from(full_key), &full_val);
}

fn set_key_from_u256(key: u32, val: U256) {
	let mut full_key = [0u8; 32];
	write_u32(&mut full_key[0..4], key);

	let _ = ext::write(&H256::from(full_key), &val.into());
}

#[no_mangle]
pub fn call() {
	set_key_from_addr(1, &ext::address());
	set_key_from_addr(2, &ext::sender());
	set_key_from_addr(3, &ext::origin());
	set_key_from_u256(4, ext::value());
}
