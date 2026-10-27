extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(_a0: *const Error, _a1: *const Error) -> () {
    return (((*_a0).i) == ((*_a1).i));
}
pub unsafe fn operator_eq_1(lhs: *const Error, mut rhs: ()) -> () {
    return (((*lhs).i) == (rhs));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Error {
    pub i: (),
}
impl std::cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const Error, other as *const Error) }
    }
}
impl std::cmp::Eq for Error {}
pub unsafe fn test_2() -> () {
    let mut unex1: () = unimplemented!();
    let mut unex2: () = unimplemented!();
    let mut unex3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut unex_i1: () = unimplemented!();
    let mut unex_i2: () = unimplemented!();
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
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
