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
    pub unsafe fn operator_add_1(&mut self, mut a: ()) -> () {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_add_2(&self, mut a: ()) -> () {
        return (((self.v) + (a)) + (1_()));
    }
    pub unsafe fn operator_add_3(&mut self, mut a: ()) -> () {
        return (((self.v) + (a)) + (2_()));
    }
    pub unsafe fn operator_sub_4(&mut self, mut a: ()) -> () {
        return ((self.v) - (a));
    }
    pub unsafe fn operator_sub_5(&mut self, mut a: ()) -> () {
        return (((self.v) - (a)) - (1_()));
    }
    pub unsafe fn operator_mul_6(&self, mut a: ()) -> () {
        return ((self.v) * (a));
    }
    pub unsafe fn operator_mul_7(&self, mut a: ()) -> () {
        return (((self.v) * (a)) * (2_()));
    }
    pub unsafe fn operator_index_8(&mut self, mut i: ()) -> () {
        return ((self.v) + (i));
    }
    pub unsafe fn operator_index_9(&self, mut i: ()) -> () {
        return (((self.v) + (i)) + (100_()));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S { v: 10_() };
    let cs: S = S { v: 10_() };
    let mut vs: S = S { v: 10_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
