#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::keccak;
use pwasm_ethereum::{ret, input};

#[no_mangle]
pub fn call() {
	ret(&keccak(&input()));
}
