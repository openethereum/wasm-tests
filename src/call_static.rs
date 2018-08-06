#![no_std]
#![allow(deprecated)]

#[macro_use] extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::{write_u32, logger};
use pwasm_std::hash::Address;
use pwasm_ethereum::{static_call, ret};
use core::hash::{SipHasher, Hasher};

#[no_mangle]
pub fn call() {
	let addr = Address::from([19u8, 7, 123, 251, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

	let input = [1u8, 2, 3, 5, 7, 11];
	let mut temp = vec![0u8; 256];
	match static_call(20000, &addr, &input, &mut temp[..]) {
		Ok(_) => {
			logger::debug("Call succeed");
		},
		Err(_) => {
			logger::debug("Call failed");
		}
	}

	let mut hasher = SipHasher::new_with_keys(127, 129);
	hasher.write(&temp[..]);
	let hash = (hasher.finish() & 0x00000000ffffffff) as u32;
	logger::debug("Hashing succed");

	let mut res = [0u8; 4];
	write_u32(&mut res[..], hash);

	logger::debug("Exiting...");
	ret(&res[..]);
}

#[no_mangle]
pub fn deploy() { }
