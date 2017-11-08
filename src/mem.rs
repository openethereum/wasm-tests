#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::Vec;

extern "C" {
	fn memcpy(dst: *mut u8, src: *const u8, len: usize) -> *mut u8;
	fn memmove(dst: *mut u8, src: *const u8, len: usize) -> *mut u8;
	fn memset(dst: *mut u8, ch: i32, len: usize) -> *mut u8;
}

#[no_mangle]
pub fn call(desc: *mut u8) {
	let (input, result) = unsafe { pwasm_std::parse_args(desc) };

	let code = input.as_ref()[0];
	let src = &input.as_ref()[1..];

	let mut dst = Vec::with_capacity(8192);
	unsafe {
		dst.set_len(8192);
	}
	match code {
		0 => unsafe {
			memcpy(dst.as_mut_ptr(), src.as_ptr(), 8192);
		},
		1 => unsafe {
			memmove(dst.as_mut_ptr(), src.as_ptr(), 8192);
		},
		2 => unsafe {
			let ch = input.as_ref()[1];
			memset(dst.as_mut_ptr(), ch as i32, 8192);
		},
		_ => unreachable!(),
	}
	result.done(dst);
}
