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
pub struct Inner {
    pub x: (),
    pub y: (),
}
impl Default for Inner {
    fn default() -> Self {
        unsafe { Inner { x: 3_(), y: 4_() } }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S {
    pub a: (),
    pub b: (),
    pub c: Inner,
    pub d: Inner,
}
impl Default for S {
    fn default() -> Self {
        unsafe {
            S {
                a: 1_(),
                b: 2_(),
                c: <Inner>::default(),
                d: <Inner>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Boxed_int_ {
    pub v: (),
    pub tag: (),
}
impl Boxed_int_ {
    pub unsafe fn new(mut x: (), mut t: ()) -> Self {
        let mut this = Self { v: x, tag: t };
        this
    }
}
impl Default for Boxed_int_ {
    fn default() -> Self {
        unsafe {
            Boxed_int_ {
                v: unimplemented!(),
                tag: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: S = <S>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut boxed: Boxed_int_ = Boxed_int_::new({ 5_() }, { 9_() });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
