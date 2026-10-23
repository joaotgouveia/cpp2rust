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
    pub unsafe fn get(&self) -> () {
        return self.v;
    }
}
pub unsafe trait Base {
    unsafe fn apply(&mut self, x: ()) -> ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Derived {
    pub factor: (),
}
impl Derived {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: (),
    pub second: (),
}
pub unsafe fn pair_diff_0(mut p: *const Pair) -> () {
    return (((*p).first) - ((*p).second));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = S::new({ 1_() });
    unimplemented!();
    (unsafe { S::set(&mut s, 4_()) });
    unimplemented!();
    unimplemented!();
    let mut derived: Derived = Derived::new({ 3_() });
    let mut base: *mut dyn Base = (&mut derived as *mut Derived);
    unimplemented!();
    let mut pair: Pair = Pair {
        first: 7_(),
        second: 3_(),
    };
    unimplemented!();
    unimplemented!();
    return 0_();
}
impl S {
    pub unsafe fn new(mut x: ()) -> Self {
        let mut this = Self { v: x };
        this
    }
}
impl Derived {
    pub unsafe fn new(mut factor: ()) -> Self {
        let mut this = Self { factor: factor };
        this
    }
}
impl S {}
impl S {
    pub unsafe fn destructor(&mut self) {}
}
impl S {
    pub unsafe fn set(&mut self, mut x: ()) {
        self.v = x;
    }
}
impl S {
    pub unsafe fn add(&mut self, mut x: ()) -> () {
        self.v += x;
        return self.v;
    }
}
impl Derived {}
pub unsafe fn pair_sum_1(mut p: *const Pair) -> () {
    return (((*p).first) + ((*p).second));
}
unsafe impl Base for Derived {
    unsafe fn apply(&mut self, x: ()) -> () {
        return ((self.factor) * (x));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
