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
impl S {
    pub unsafe fn to__1(&self) -> () {
        return self.v;
    }
    pub unsafe fn to__2(&self) -> () {
        return ((self.v) + (1_()));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct T {
    pub v: (),
}
impl T {
    pub unsafe fn to__1(&self) -> () {
        return self.v;
    }
    pub unsafe fn to__2(&self) -> () {
        return ((self.v) + (1_()));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { v: 65_() };
    let mut a: () = (unsafe { S::to__1(&s) });
    let mut b: () = (unsafe { S::to__2(&s) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut t: T = T { v: 3_() };
    let mut c: () = (unsafe { T::to__1(&t) });
    let mut d: () = (unsafe { T::to__2(&t) });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
