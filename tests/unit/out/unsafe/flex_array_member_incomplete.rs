extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct F {
    pub n: (),
    pub tail: [(); 0],
}
impl Default for F {
    fn default() -> Self {
        unsafe {
            F {
                n: unimplemented!(),
                tail: [],
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    let mut f: *mut F = (unimplemented!() as *mut F);
    unimplemented!();
    (*f).n = 4_();
    (unsafe {
        memcpy_0(
            ((*f).tail.as_mut_ptr() as ()),
            (c"xyz".as_ptr().cast_mut() as ()),
            4_(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut g: *mut F = (unimplemented!() as *mut F);
    unimplemented!();
    (*g).n = 8_();
    (unsafe {
        memcpy_0(
            ((*g).tail.as_mut_ptr() as ()),
            (c"abcdefg".as_ptr().cast_mut() as ()),
            8_(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
