#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_std::Vec;
use pwasm_ethereum::{input, ret};

#[no_mangle]
pub fn call() {
    let input = input();

    let mut dispersed = Vec::with_capacity(input.len() * 2);
    for e in input.iter() {
        dispersed.push(*e);
        dispersed.push(e % 19);
    }

    ret(&dispersed)
}