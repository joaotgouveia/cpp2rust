extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A {
    pub v: (),
}
impl A {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { v: 1_() };
        this
    }
    pub unsafe fn new_2(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new_1() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct B {
    pub v: (),
}
impl B {
    pub unsafe fn new() -> Self {
        let mut this = Self { v: 2_() };
        this
    }
}
impl Default for B {
    fn default() -> Self {
        unsafe { B::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoDefault {
    pub v: (),
}
impl NoDefault {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
}
pub unsafe fn used_0(mut x: Option<A>) -> () {
    let mut x: A = x.unwrap_or_else(|| unsafe { A::new_1() });
    return x.v;
}
pub unsafe fn used_1(mut x: Option<B>) -> () {
    let mut x: B = x.unwrap_or_else(|| unsafe { B::new() });
    return x.v;
}
pub unsafe fn scaled_2(mut x: A, mut n: Option<()>) -> () {
    let mut n: () = n.unwrap_or_else(|| unsafe { ::std::mem::size_of::<A>() });
    return ((x.v) * (n));
}
pub unsafe fn always_given_3(mut x: NoDefault) -> () {
    return x.v;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_NoDefault_ {
    pub v: (),
}
impl S_NoDefault_ {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn get(&self, mut t: NoDefault) -> () {
        return ((self.v) + (t.v));
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: S_NoDefault_ = S_NoDefault_::new({ 1_() });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
