#![no_std]

extern crate uint;

extern crate pwasm_ethereum;

use pwasm_ethereum::{ret, input};
use uint::U256;

#[no_mangle]
pub fn call() {
	let input = input();

	ret(&{
		let code = input[0];

		let a_param: U256 = (&input[1..33]).into();
		let b_param: U256 = (&input[33..65]).into();

		let result = match code {
			0 => { a_param + b_param },
			1 => { a_param * b_param },
			2 => { a_param - b_param },
			_ => { a_param / b_param },
		};

		let mut result_bytes = [0u8; 32];
		result.to_big_endian(&mut result_bytes);

		result_bytes
	})
}
