
#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_ethereum::{suicide, ret, input};

#[no_mangle]
pub fn call() {
	let input = input();

	if input.len() > 0 && input[0] == 127 {
		let mut addr = [0u8; 20];
		addr.copy_from_slice(&input[1..]);
		suicide(&addr.into());
	} else {
		ret(&input);
	}
}
