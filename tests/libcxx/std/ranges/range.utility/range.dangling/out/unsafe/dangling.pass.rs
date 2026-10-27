extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(true, "std::is_empty_v<std::ranges::dangling>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_0_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_1_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_2_ {}
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling, S<0>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling, S<0>, S<1>>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_constructible_v<std::ranges::dangling, S<0>, S<1>, S<2>>"
);
pub unsafe fn test_dangling_0() -> () {
    let mut a: () = unimplemented!();
    let mut b: () = unimplemented!();
    let mut c: () = unimplemented!();
    let mut d: () = unimplemented!();
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
    const _: () = assert!(true, "test_dangling()");;
    (unsafe { test_dangling_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
