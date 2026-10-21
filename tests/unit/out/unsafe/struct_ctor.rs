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
pub struct StructWithCtor {
    x1_: (),
    x2_: (),
}
impl StructWithCtor {
    pub unsafe fn new(mut x1: (), mut x2: ()) -> Self {
        let mut this = Self { x1_: x1, x2_: x2 };
        this.x1_.prefix_inc();
        this.x2_.prefix_dec();
        this
    }
    pub unsafe fn x1(&self) -> *const () {
        return &self.x1_;
    }
    pub unsafe fn x2(&self) -> *const () {
        return &self.x2_;
    }
}
pub unsafe fn foo_0(x: *mut ()) -> *mut () {
    return x;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Value_ {
    pub v: (),
}
impl Value_ {
    pub unsafe fn new(mut u: ()) -> Self {
        let mut this = Self { v: u };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Ptr_ {
    pub v1: Value_,
    pub v2: Value_,
}
impl Ptr_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            v1: Value_::new({ 11_() }),
            v2: Value_::new({ 22_() }),
        };
        this
    }
}
impl Default for Ptr_ {
    fn default() -> Self {
        unsafe { Ptr_::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut struct_with_ctor: StructWithCtor = StructWithCtor::new({ 1_() }, { 2_() });
    let mut x: () = 3_();
    unimplemented!();
    let mut p: Ptr_ = Ptr_::new();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
