#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum as ext;

use pwasm_std::hash::H256;

#[no_mangle]
pub fn call() {
	let input = ext::input();

	for i in 0..input.len() / 64 {
		let key = H256::from(&ext::input()[0+i*64..32+i*64]);
		let mut val = [0u8;32];
		val.copy_from_slice(&ext::input()[32+i*64..64+i*64]);
		ext::write(&key, &val);
	}
}