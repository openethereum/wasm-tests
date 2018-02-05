#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum as ext;

use pwasm_std::Vec;

#[no_mangle]
pub fn call() {
	ext::ret(&{
		let mut data = Vec::with_capacity(450 * 1024);
		data.extend_from_slice(&[5u8; 450*1024][..]);
		data
	});
}