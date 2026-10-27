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
pub struct A {
    i_: (),
    c_: (),
}
impl A {
    pub unsafe fn new(mut i: (), mut c: ()) -> Self {
        let mut this = Self { i_: i, c_: c };
        this
    }
    pub unsafe fn get_i(&self) -> () {
        return self.i_;
    }
    pub unsafe fn get_c(&self) -> () {
        return self.c_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B {
    d_: (),
    u1_: (),
    u2_: (),
}
impl B {
    pub unsafe fn new(mut d: (), mut u1: (), mut u2: ()) -> Self {
        let mut this = Self {
            d_: d,
            u1_: u1,
            u2_: u2,
        };
        this
    }
    pub unsafe fn get_d(&self) -> () {
        return self.d_;
    }
    pub unsafe fn get_u1(&self) -> () {
        return self.u1_;
    }
    pub unsafe fn get_u2(&self) -> () {
        return self.u2_;
    }
}
pub unsafe fn test_0() -> () {
    let mut p: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
