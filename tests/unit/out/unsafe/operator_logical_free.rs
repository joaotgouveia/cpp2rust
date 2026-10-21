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
    pub v: (),
}
pub unsafe fn operator_not_0(a: *const S) -> () {
    return (((*a).v) == (unimplemented!()));
}
pub unsafe fn operator_and_1(a: *const S, b: *const S) -> () {
    return (((*a).v) != (unimplemented!())) && (((*b).v) != (unimplemented!()));
}
pub unsafe fn operator_or_2(a: *const S, b: *const S) -> () {
    return (((*a).v) != (unimplemented!())) || (((*b).v) != (unimplemented!()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut t: S = S { v: 1_() };
    let mut f: S = S {
        v: unimplemented!(),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
