#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::Vec;
use pwasm_ethereum::{input, ret};

#[no_mangle]
pub fn call() {
	let mut input = input();
	unsafe {
		let cap = input.capacity();
		input.set_len(cap);
	}

	// let mut dispersed = Vec::with_capacity(input.len() * 2);
	let mut dispersed = Vec::with_capacity(5);

	for e in input.iter() {
		 dispersed.push(*e);
		 dispersed.push(e % 19);
	}

	ret(&dispersed[..])
}