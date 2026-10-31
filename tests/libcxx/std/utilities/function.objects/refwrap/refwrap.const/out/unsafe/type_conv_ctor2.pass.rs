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
pub unsafe fn test_0() -> bool {
    {
        let mut a: A1 = <A1>::default();
        const _: () = assert!(true, "");;
        let mut b1: *mut B = &mut a;
        assert!(((b1) == (&mut a.b_ as *mut B)));
        const _: () = assert!(true, "");;
        unimplemented!();
        assert!(((b1) == (&mut a.b_ as *mut B)));
    }
    {
        let mut a: A2 = <A2>::default();
        const _: () = assert!(true, "");;
        let mut b2: *mut B = &mut a;
        assert!(((b2) == (&mut a.b_ as *mut B)));
        const _: () = assert!(true, "");;
        unimplemented!();
        assert!(((b2) == (&mut a.b_ as *mut B)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
