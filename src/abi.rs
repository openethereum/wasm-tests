#![no_main]
#![no_std]

extern crate pwasm_std;
extern crate pwasm_abi;
#[macro_use] extern crate pwasm_abi_derive;

use pwasm_std::CallArgs;
use pwasm_abi_derive::legacy_dispatch;

mod contract {
    use pwasm_std::CallArgs;

    #[legacy_dispatch]
    trait Interface {
        fn baz(&mut self, p1: u32, p2: bool);
    }

    struct Instance {
        pub args: CallArgs,
    }

    impl Interface for Instance {
        fn baz(&mut self, p1: u32, p2: bool);
    }

    impl Instance {
        fn new(args: CallArgs) -> Self {
            Instance { args: args }
        }
    }

    impl Endpoint {
        fn new(args: CallArgs) -> Self {
            Endpoint::new(Instance::new(args))
        }
    }
}

#[no_mangle]
pub fn call(desc: *mut u8) {
    let endpoint = contract::Endpoint::new(unsafe { CallArgs::from_raw(desc) });
    endpoint.dispatch
}

#[no_mangle]
pub fn create(_desc: *mut u8) {

}