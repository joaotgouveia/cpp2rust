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
pub struct base {
    pub kind: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct derived {
    pub head: base,
    pub value: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut d: *mut derived = (unimplemented!() as *mut derived);
    unimplemented!();
    (*d).head.kind = 3_();
    (*d).value = 7_();
    let mut b: *mut base = (&mut (*d).head as *mut base);
    let mut back: *mut derived = (b as *mut derived);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*back).value = 8_();
    unimplemented!();
    (*b).kind = 4_();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
