#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum as ext;

use pwasm_std::keccak;
use pwasm_std::hash::H256;

#[no_mangle]
pub fn call() {
	let input = ext::input();

	let hash = keccak(&input);
	let mut reverse_hash = hash.to_vec();
	reverse_hash.reverse();

	let mut reverse_input = input.to_vec();
	reverse_input.reverse();

	ext::log(&[hash, H256::from(&reverse_hash[..])], &reverse_input);

	ext::ret(&reverse_input);
}