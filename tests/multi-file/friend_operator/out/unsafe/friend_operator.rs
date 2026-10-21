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
    pub b: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s1: S = S { a: 1_(), b: 2_() };
    let mut s2: S = S { a: 1_(), b: 2_() };
    let mut s3: S = S { a: 1_(), b: 3_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn operator_eq_0(x: *const S, y: *const S) -> () {
    return (((*x).a) == ((*y).a)) && (((*x).b) == ((*y).b));
}
pub unsafe fn operator_ne_1(x: *const S, y: *const S) -> () {
    return !(unsafe {
        let _x: *const S = x;
        let _y: *const S = y;
        operator_eq_0(_x, _y)
    });
}
pub unsafe fn operator_lt_2(x: *const S, y: *const S) -> () {
    return (((*x).a) < ((*y).a)) || ((((*x).a) == ((*y).a)) && (((*x).b) < ((*y).b)));
}
pub unsafe fn compare_3(x: *const S, y: *const S) -> () {
    if (unsafe {
        let _x: *const S = x;
        let _y: *const S = y;
        operator_lt_2(_x, _y)
    }) {
        return -1_();
    }
    if (unsafe {
        let _x: *const S = y;
        let _y: *const S = x;
        operator_lt_2(_x, _y)
    }) {
        return 1_();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
