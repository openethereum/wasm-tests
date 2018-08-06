#![no_std]

extern crate pwasm_std;

use pwasm_std::logger;

#[no_mangle]
pub fn call() {
	logger::debug("Empty contract");
}

#[no_mangle]
pub fn deploy() { }
