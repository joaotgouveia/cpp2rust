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
pub struct B {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A1 {
    pub b_: B,
}
impl A1 {
    pub unsafe fn to_pmutB(&self) -> *mut B {
        return &mut self.b_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A2 {
    pub b_: B,
}
impl A2 {
    pub unsafe fn to_pmutB(&self) -> *mut B {
        return &mut self.b_;
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut a: A1 = <A1>::default();
        const _: () = assert!(true, "");;
        let mut b1: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        b1 = unimplemented!();
        unimplemented!();
    }
    {
        let mut a: A2 = <A2>::default();
        const _: () = assert!(true, "");;
        let mut b2: () = unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "");;
        b2 = unimplemented!();
        unimplemented!();
    }
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
