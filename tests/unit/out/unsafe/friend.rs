extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn get_0(v: *const V) -> () {
    return (*v).x;
}
pub unsafe fn operator_eq_1(a: *const V, b: *const V) -> () {
    return (((*a).x) == ((*b).x));
}
pub unsafe fn scaled_2(v: *const V, mut k: ()) -> () {
    return (((*v).x) * (k));
}
pub unsafe fn scaled_3(v: *const V, mut k: ()) -> () {
    return (((*v).x) * (k));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct V {
    pub x: (),
}
impl std::cmp::PartialEq for V {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const V, other as *const V) }
    }
}
impl std::cmp::Eq for V {}
pub unsafe fn get_4(w: *const W_int_) -> () {
    return (*w).x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct W_int_ {
    pub x: (),
}
pub unsafe fn get_5(w: *const W_long_) -> () {
    return (*w).x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct W_long_ {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct D {
    pub x: (),
}
pub unsafe fn declared_then_defined_6(d: *const D) -> () {
    return (((*d).x) + (1_()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: V = V { x: 3_() };
    let mut b: V = V { x: 3_() };
    let mut c: V = V { x: 4_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut wi: W_int_ = W_int_ { x: 5_() };
    let mut wl: W_long_ = W_long_ { x: 6_() };
    unimplemented!();
    unimplemented!();
    let mut d: D = D { x: 7_() };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
