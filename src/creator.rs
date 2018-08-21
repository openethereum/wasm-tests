#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;
extern crate uint;

extern crate parity_hash as hash;

use pwasm_std::logger;
use pwasm_ethereum::{input, ret, create, create2, value};

use uint::U256;
use hash::H256;

#[no_mangle]
pub fn call() {
	let mut r = [0u8; 40];
	if let Ok(addr) = create(value() / U256::from(2), &input()) {
		logger::debug("Created contract with code");
		(&mut r[0..20]).copy_from_slice(&addr[..]);
	} else {
		logger::debug("Error creating contract");
	}

	if let Ok(addr) = create2(value() / U256::from(2), H256::from([5u8].as_ref()), &input()) {
		logger::debug("Created contract with code and salt");
		(&mut r[0..20]).copy_from_slice(&addr[..]);
	} else {
		logger::debug("Error creating contract");
	}

	ret(&r[..]);
}
