#![no_main]
#![no_std]

extern crate pwasm_std;
extern crate pwasm_ethereum;

use pwasm_ethereum::ext;

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (input, result) = unsafe { pwasm_std::parse_args(desc) };

    if input.as_ref().len() > 0 && input.as_ref()[0] == 127 {
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&input.as_ref()[1..]);
        ext::suicide(&addr.into());
    } else {
        result.done(input.as_ref().to_vec());
    }
}
