#![no_main]
#![no_std]

extern crate pwasm_std;
extern crate bigint;

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (input, result) = unsafe { pwasm_std::parse_args(desc) };

    result.done({
        let code = input.as_ref()[0];

        let a_param: bigint::U256 = (&input.as_ref()[1..33]).into();
        let b_param: bigint::U256 = (&input.as_ref()[33..65]).into();

        let result = match code {
            0 => { a_param + b_param },
            1 => { a_param * b_param },
            2 => { a_param - b_param },
            _ => { a_param / b_param },
        };

        let mut result_bytes = [0u8; 32];
        result.to_big_endian(&mut result_bytes);

        result_bytes
    }.to_vec())
}