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
pub struct Point {
    pub x: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_int_ {
    pub val: (),
}
impl Box_int_ {
    pub unsafe fn twice(&mut self) -> () {
        return ((self.val) + (self.val));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Box_Point_ {
    pub val: Point,
}
impl Box_Point_ {
    pub unsafe fn get(&mut self) -> Point {
        return self.val;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Deduced_int_ {
    pub val: (),
}
impl Deduced_int_ {
    pub unsafe fn twice(&mut self) -> () {
        return ((self.val) + (self.val));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Deduced_Point_ {
    pub val: Point,
}
impl Deduced_Point_ {
    pub unsafe fn get(&mut self) -> Point {
        return self.val;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i: Box_int_ = Box_int_ { val: 3_() };
    unimplemented!();
    let mut p: Box_Point_ = Box_Point_ {
        val: Point { x: 4_() },
    };
    unimplemented!();
    let mut d: Deduced_int_ = Deduced_int_ { val: 5_() };
    unimplemented!();
    let mut dp: Deduced_Point_ = Deduced_Point_ {
        val: Point { x: 6_() },
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
