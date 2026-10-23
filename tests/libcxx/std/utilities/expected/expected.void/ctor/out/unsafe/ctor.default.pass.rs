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
pub struct NoDefaultCtor {}
const _: () = assert!(
    true,
    "std::is_nothrow_default_constructible_v<std::expected<void, int>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_default_constructible_v<std::expected<void, NoDefaultCtor>>"
);
pub unsafe fn operator_eq_0(_a0: *const MyInt, _a1: *const MyInt) -> () {
    return (((*_a0).i) == ((*_a1).i));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyInt {
    pub i: (),
}
impl std::cmp::PartialEq for MyInt {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const MyInt, other as *const MyInt) }
    }
}
impl std::cmp::Eq for MyInt {}
pub unsafe fn test_1() -> () {
    {
        let mut e: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut e: () = unimplemented!();
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
