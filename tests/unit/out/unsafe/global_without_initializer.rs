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
pub struct S {
    pub a: (),
}
pub static mut s_0: std::cell::LazyCell<*mut S> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut file_1: std::cell::LazyCell<*mut ::libc::FILE> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
pub static mut size_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const s_0);
    std::cell::LazyCell::force(&*&raw const file_1);
    std::cell::LazyCell::force(&*&raw const size_2);
}
