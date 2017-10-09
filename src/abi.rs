#![no_main]
#![no_std]
#![feature(proc_macro)]
#![feature(alloc)]

extern crate pwasm_std;
extern crate pwasm_abi;
extern crate pwasm_abi_derive;
extern crate alloc;


mod contract {
    use pwasm_std::Vec;
    use pwasm_abi_derive::legacy_dispatch;
    use alloc::borrow::Cow;

    #[legacy_dispatch(Endpoint)]
    pub trait Interface {
        fn baz(&mut self, p1: u32, p2: bool) -> bool;
    }

    pub struct Instance;

    impl Interface for Instance {
        fn baz(&mut self, p1: u32, p2: bool) -> bool {
            p2 && !(p1 == 0)
        }
    }
}

#[no_mangle]
pub fn call(desc: *mut u8) {
    let (input, result) = unsafe { pwasm_std::parse_args(desc) };
    let mut endpoint = contract::Endpoint::new(contract::Instance);
    result.done(endpoint.dispatch(&*input))
}

#[no_mangle]
pub fn create(desc: *mut u8) {
    let (input, _) = unsafe { pwasm_std::parse_args(desc) };
    let mut endpoint = contract::Endpoint::new(contract::Instance);
    endpoint.dispatch_ctor(&*input);
}