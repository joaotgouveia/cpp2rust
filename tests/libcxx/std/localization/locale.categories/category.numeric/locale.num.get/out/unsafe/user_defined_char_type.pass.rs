extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Char {
    pub underlying_: (),
}
impl Char {
    pub unsafe fn new_1(mut c: ()) -> Self {
        let mut this = Self { underlying_: c };
        this
    }
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { underlying_: i };
        this
    }
    pub unsafe fn new_3(mut i: i32) -> Self {
        let mut this = Self {
            underlying_: (i as ()),
        };
        this
    }
    pub unsafe fn to_i32(&self) -> i32 {
        return (self.underlying_ as i32);
    }
}
pub static mut id_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut id_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });

pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    let mut l: () = unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const id_0);
    std::cell::LazyCell::force(&*&raw const id_1);
}
