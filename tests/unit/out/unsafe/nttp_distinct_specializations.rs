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
pub struct S_2_ {
    pub v: (),
    pub w: (),
}
impl S_2_ {
    pub unsafe fn get(&self) -> () {
        return ((self.v) - (self.w));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_1_ {
    pub v: (),
}
impl S_1_ {
    pub unsafe fn get(&self) -> () {
        return ((self.v) * (1_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_3_ {
    pub v: (),
}
impl S_3_ {
    pub unsafe fn get(&self) -> () {
        return ((self.v) * (3_()));
    }
}
pub unsafe fn scale_0(mut x: ()) -> () {
    return ((x) * (1_()));
}
pub unsafe fn scale_1(mut x: ()) -> () {
    return ((x) * (3_()));
}
pub unsafe fn scale_2(mut x: ()) -> () {
    return ((x) - (2_()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s1: S_1_ = S_1_ { v: 5_() };
    let mut s2: S_2_ = S_2_ { v: 5_(), w: 1_() };
    let mut s3: S_3_ = S_3_ { v: 5_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
