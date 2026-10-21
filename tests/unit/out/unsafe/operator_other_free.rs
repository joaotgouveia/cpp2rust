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
pub unsafe fn operator_comma_0(a: *const S, b: *const S) -> S {
    return S {
        v: ((((*a).v) * (10_())) + ((*b).v)),
    };
}
pub unsafe fn operator_literal__k_1(mut v: ()) -> () {
    return (v).wrapping_mul(1000_());
}
pub unsafe fn operator_literal__half_2(mut v: ()) -> () {
    return ((v) / (2_()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { v: 3_() };
    let mut t: S = S { v: 4_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
