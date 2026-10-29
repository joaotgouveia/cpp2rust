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
    pub keep: (),
    pub a: (),
    pub b: (),
    pub c: [(); 5],
    pub last: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    unimplemented!();
    (*p).keep = 7_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*p).a = 1_();
    (*p).b = 2_();
    (*p).c[(0_()) as usize] = ('x' as ());
    (*p).last = 3_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
