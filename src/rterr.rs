#![no_std]

extern crate pwasm_std;

use pwasm_std::logger;

#[no_mangle]
pub fn call() {
	logger::debug("Exception will occur here:");
	unreachable!();
}