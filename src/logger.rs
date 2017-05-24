#![feature(link_args)]
#![no_main]

mod helpers;

use helpers::{CallArgs, storage};

#[no_mangle]
pub fn call(descriptor: *mut u8) {
    // This initializes safe wrapper for contract input and output
    let mut ctx = CallArgs::from_raw(descriptor);

    // Copies all contract input data to the separate buffer
    let data = ctx.context().to_vec();

    let storage_key = [1u8; 32];
    let mut storage_val = [0u8; 32];
    storage_val.copy_from_slice(&data[0..32]);

    // Sets the key [1, 1, 1 ..., 1] to the first 32 bytes of passed input
    let _ = storage::write(&storage_key, &mut storage_val);

    // Returns all that passed to this contract as an output
    *ctx.result_mut() = data;

    // Saves the wrapper state to commit return stream
    ctx.save(descriptor);
}