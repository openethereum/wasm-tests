#![allow(dead_code)] // current issues of the build process

use std::{self, slice};

#[link_args = "-s NO_EXIT_RUNTIME=1 -s NO_FILESYSTEM=1 -s WASM=1"]
extern {}

/// Wrapper over storage read/write externs
/// Storage api is a key-value storage where both key and value are 32 bytes in len
pub mod storage {
    pub struct Error;

    #[link(name = "env")]
    extern {
        fn storage_read(key: *const u8, dst: *mut u8) -> i32;
        fn storage_write(key: *const u8, src: *const u8) -> i32;
    }

    /// Performs read from storage to the specified slice `dst`
    /// Can return `Error` if data is read from outside of the storage boundaries
    pub fn read(key: &[u8; 32], dst: &mut [u8; 32]) -> Result<(), Error> {
        match unsafe {
            let mut dst = dst;
            storage_read(key.as_ptr(), dst.as_mut_ptr())
        } {
            x if x < 0 => Err(Error),
            _ => Ok(()),
        }
    }

    /// Performs write to the storage from the specified `src`
    pub fn write(key: &[u8; 32], src: &[u8; 32]) -> Result<(), Error> {
        match unsafe {
            storage_write(key.as_ptr(), src.as_ptr())
        } {
            x if x < 0 => Err(Error),
            _ => Ok(()),
        }
    }
}

/// Safe wrapper around debug logging
pub mod logger {
    mod external {
        #[link(name = "env")]
        extern {
            pub fn debug(str_ptr: *const u8, str_len: u32);
        }
    }

    pub fn debug(msg: &str) {
        unsafe { external::debug(msg.as_ptr(), msg.len() as u32); }
    }

}

/// Safe wrapper around externalities invokes
pub mod ext {
    pub struct Error;

    mod external {
        #[link(name = "env")]
        extern {
            pub fn suicide(refund: *const u8);
        }
        #[link(name = "env")]
        extern {
            pub fn create(endowment: *const u8, code_ptr: *const u8, code_len: u32, result_ptr: *mut u8) -> i32;
        }
    }

    pub fn suicide(refund: &[u8; 20]) {
        unsafe { external::suicide(refund.as_ptr()); }
    }

    pub fn create(endowment: &[u8; 32], code: &[u8]) -> Result<[u8; 20], Error> {
        let mut result = [0u8; 20];
        unsafe {
            if external::create(endowment.as_ptr(), code.as_ptr(), code.len() as u32, (&mut result).as_mut_ptr()) == 0 {
                Ok(result)
            } else {
                Err(Error)
            }
        }
    }
}

/// Safe wrapper for call context
pub struct CallArgs {
    context: Box<[u8]>,
    result: Vec<u8>,
}

unsafe fn read_ptr_mut(slc: &[u8]) -> *mut u8 {
    std::ptr::null_mut().offset(read_u32(slc) as isize)
}

pub fn read_u32(slc: &[u8]) -> u32 {
    use std::ops::Shl;
    (slc[0] as u32) + (slc[1] as u32).shl(8) + (slc[2] as u32).shl(16) + (slc[3] as u32).shl(24)
}

pub fn write_u32(dst: &mut [u8], val: u32) {
    dst[0] = (val & 0x000000ff) as u8;
    dst[1] = ((val & 0x0000ff00) >> 8) as u8;
    dst[2] = ((val & 0x00ff0000) >> 16) as u8;
    dst[3] = ((val & 0xff000000) >> 24) as u8;
}

fn write_ptr(dst: &mut [u8], ptr: *mut u8) {
    // todo: consider: add assert that arch is 32bit
    write_u32(dst, ptr as usize as u32);
}

impl CallArgs {
    pub unsafe fn from_raw(ptr: *mut u8) -> CallArgs {
        let desc_slice = slice::from_raw_parts(ptr, 4 * 4);

        let context_ptr = read_ptr_mut(&desc_slice[0..4]);
        let context_len = read_u32(&desc_slice[4..8]) as usize;

        let result_ptr = read_ptr_mut(&desc_slice[8..12]);
        let result_len = read_u32(&desc_slice[12..16]) as usize;

        CallArgs {
            context: Box::<[u8]>::from_raw(slice::from_raw_parts_mut(context_ptr, context_len)),
            result: Vec::from_raw_parts(result_ptr, result_len, result_len),
        }
    }

    pub fn context(&self) -> &[u8] {
        &self.context
    }

    pub fn result_mut(&mut self) -> &mut Vec<u8> {
        &mut self.result
    }

    pub unsafe fn save(self, ptr: *mut u8) {
        let dst = slice::from_raw_parts_mut(ptr.offset(8), 2 * 4);
        let context = self.context;
        let mut result = self.result;

        // context unmodified and memory is managed in calling code
        std::mem::forget(context);

        if result.len() > 0 {
            // result
            write_ptr(&mut dst[0..4], result.as_mut_ptr());
            write_u32(&mut dst[4..8], result.len() as u32);
            // managed in calling code
            std::mem::forget(result);
        }
    }

    pub fn params<'a> (&'a self) -> ParamsView<'a> {
        ParamsView::new(self.context())
    }
}

pub struct ParamsView<'a> {
    raw: &'a [u8],
}

impl<'a> ParamsView<'a> {
    fn new(raw: &'a [u8]) -> Self {
        ParamsView { raw: raw }
    }

    pub fn address(&self) -> [u8; 20] {
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&self.raw[0..20]);
        addr
    }

    pub fn sender(&self) -> [u8; 20] {
        let mut sender = [0u8; 20];
        sender.copy_from_slice(&self.raw[20..40]);
        sender
    }

    pub fn origin(&self) -> [u8; 20] {
        let mut origin = [0u8; 20];
        origin.copy_from_slice(&self.raw[40..60]);
        origin
    }

    pub fn value(&self) -> [u8; 32] {
        let mut value = [0u8; 32];
        value.copy_from_slice(&self.raw[60..92]);
        value
    }

    pub fn args(&self) -> &[u8] {
        &self.raw[92..]
    }
}