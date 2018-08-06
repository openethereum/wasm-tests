#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::logger;
use pwasm_ethereum::{input, ret, create, value};

#[no_mangle]
pub fn call() {
	if let Ok(addr) = create(value(), &input()) {
		logger::debug("Created contractwith code");
		ret(&addr[..]);
	} else {
		logger::debug("Error creating contract");
	}
}

#[no_mangle]
pub fn deploy() { }
