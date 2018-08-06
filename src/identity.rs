#![no_std]

extern crate pwasm_ethereum as ext;

#[no_mangle]
pub fn call() {
	ext::ret(&ext::sender()[..]);
}

#[no_mangle]
pub fn deploy() { }
