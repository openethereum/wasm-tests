#![no_main]
#![no_std]

extern crate pwasm_std;

use pwasm_std::{ext, Vec, CallArgs, write_u64};

fn push_u64(buf: &mut Vec<u8>, val: u64) {
    let mut slc = [0u8; 8];
    write_u64(&mut slc, val);
    buf.extend(&slc[..]);
}

#[no_mangle]
pub fn call(desc: *mut u8) {
    let mut ctx = unsafe { CallArgs::from_raw(desc) };

    let mut output: Vec<u8> = Vec::with_capacity(64 + 20 + 8 + 8 + 32 + 32);

    output.extend(&ext::block_hash(0).expect("block_hash to return hash for b0")[..]);
    output.extend(&ext::block_hash(1).expect("block_hash to return hash for b1")[..]);
    output.extend(&ext::coinbase()[..]);
    push_u64(&mut output, ext::timestamp());
    push_u64(&mut output, ext::block_number());
    output.extend(&ext::difficulty()[..]);
    output.extend(&ext::gas_limit()[..]);

    *ctx.result_mut() = output.into_boxed_slice();

    unsafe { ctx.save(desc); }
}