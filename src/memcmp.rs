#![no_main]
#![no_std]

#[macro_use]
extern crate pwasm_std;

use pwasm_std::Vec;

extern "C" {
	fn memcmp(cx: *const u8, ct: *const u8, n: usize) -> i32;
}

#[no_mangle]
pub fn call(desc: *mut u8) {
	let (input, result) = unsafe { pwasm_std::parse_args(desc) };

	let result_cmp = unsafe { memcmp(input.as_ptr(), vec![1u8, 1, 1].as_ptr(), 3) };

	let res: Vec<u8> = vec![
		(result_cmp as u32 & 0x000000ff) as u8,
		((result_cmp as u32 & 0x0000ff00) >> 8) as u8,
		((result_cmp as u32 & 0x00ff0000) >> 16) as u8,
		((result_cmp as u32 & 0xff000000) >> 24) as u8,
	];

	result.done(res);
}
