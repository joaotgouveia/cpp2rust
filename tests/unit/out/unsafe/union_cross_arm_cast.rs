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
pub struct shape_a {
    pub code: (),
    pub pad: [(); 14],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct shape_b {
    pub code: (),
    pub lo: (),
    pub mid: (),
    pub fill: [(); 16],
    pub tail: (),
}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union anon_0 {
    pub a: shape_a,
    pub b: shape_b,
    pub raw_: [(); 64],
}
impl Default for anon_0 {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Container {
    pub len: (),
    pub u: anon_0,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: Container = <Container>::default();
    (unsafe {
        let ___s: () = ((&mut c as *mut Container) as ());
        let ___n: () = ::std::mem::size_of::<Container>();
        memset_1(___s, unimplemented!(), ___n)
    });
    c.u.a.code = 10_();
    c.len = ::std::mem::size_of::<shape_b>();
    (*(((&mut c.u.a as *mut shape_a) as ()) as *mut shape_b)).tail = 3735928559_();
    unimplemented!();
    unimplemented!();
    c.u.b.lo = 8080_();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
