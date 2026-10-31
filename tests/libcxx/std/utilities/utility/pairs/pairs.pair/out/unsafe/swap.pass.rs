extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_from(i32, new_1, value)]
pub struct S {
    pub i: i32,
}
impl S {
    pub unsafe fn new_2() -> Self {
        let mut this = Self { i: 0 };
        this
    }
    pub unsafe fn new_1(mut j: i32) -> Self {
        let mut this = Self { i: j };
        this
    }
    pub unsafe fn operator_eq(&self, mut x: i32) -> bool {
        return ((self.i) == (x));
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new_2() }
    }
}
pub unsafe fn test_0() -> bool {
    {
        let mut p1: (i32, i16) = (3, 4_i16);
        let mut p2: (i32, i16) = (5, 6_i16);
        unimplemented!();
        assert!(((p1.0) == (5)));
        assert!(((p1.1 as i32) == (6)));
        assert!(((p2.0) == (3)));
        assert!(((p2.1 as i32) == (4)));
    }
    {
        let mut p1: (i32, S) = (3, S::new_1({ 4 }));
        let mut p2: (i32, S) = (5, S::new_1({ 6 }));
        unimplemented!();
        assert!(((p1.0) == (5)));
        assert!((unsafe { S::operator_eq(&p1.1, 6,) }));
        assert!(((p2.0) == (3)));
        assert!((unsafe { S::operator_eq(&p2.1, 4,) }));
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
