#![no_std]

#[macro_use] extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::logger;
use pwasm_std::hash::Address;
use pwasm_ethereum::{call as ext_call, ret};

#[no_mangle]
pub fn call() {
    let addr = Address::from([99, 88, 77, 66, 55, 44, 33, 22, 11, 0, 11, 22, 33, 44, 55, 66, 77, 88, 99, 0]);

    let input = [129u8, 123, 113, 107, 101, 97];
    let mut temp = vec![0u8; 256];
    match ext_call(33000, &addr, 1000000000.into(), &input, &mut temp[..]) {
        Ok(_) => {
            logger::debug("Call succeed");
        },
        Err(_) => {
            logger::debug("Call failed");
        }
    }

    ret(&[]);
}
