#![no_main]
#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_ethereum::ext::log;
use pwasm_std::keccak;
use pwasm_std::hash::H256;

#[no_mangle]
pub fn call(desc: *mut u8) {
	let (input, result) = unsafe { pwasm_std::parse_args(desc) };

	let hash = keccak(&input);
	let mut reverse_hash = hash.to_vec();
	reverse_hash.reverse();

	let mut reverse_input = input.to_vec();
	reverse_input.reverse();

	log(&[hash, H256::from(&reverse_hash[..])], &reverse_input);

	result.done(reverse_input);
}