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
    i_: i32,
    c_: libc::c_char,
}
impl A {
    pub unsafe fn new(mut i: i32, mut c: libc::c_char) -> Self {
        let mut this = Self { i_: i, c_: c };
        this
    }
    pub unsafe fn get_i(&self) -> i32 {
        return self.i_;
    }
    pub unsafe fn get_c(&self) -> libc::c_char {
        return self.c_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B {
    d_: f64,
    u1_: u32,
    u2_: u32,
}
impl B {
    pub unsafe fn new(mut d: f64, mut u1: u32, mut u2: u32) -> Self {
        let mut this = Self {
            d_: d,
            u1_: u1,
            u2_: u2,
        };
        this
    }
    pub unsafe fn get_d(&self) -> f64 {
        return self.d_;
    }
    pub unsafe fn get_u1(&self) -> u32 {
        return self.u1_;
    }
    pub unsafe fn get_u2(&self) -> u32 {
        return self.u2_;
    }
}
pub unsafe fn test_0() -> bool {
    let mut p: (A, B) = unimplemented!();
    assert!(((unsafe { A::get_i(&p.0,) }) == (4)));
    assert!((((unsafe { A::get_c(&p.0,) }) as i32) == (('a' as libc::c_char) as i32)));
    assert!(((unsafe { B::get_d(&p.1,) }) == (3.5_f64)));
    assert!(((unsafe { B::get_u1(&p.1,) }) == (6_u32)));
    assert!(((unsafe { B::get_u2(&p.1,) }) == (2_u32)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
