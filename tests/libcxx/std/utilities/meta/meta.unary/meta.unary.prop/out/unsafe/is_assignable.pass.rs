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
pub struct A {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct B {}
pub unsafe fn test_is_assignable_0() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_assignable_1() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_assignable_2() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_assignable_3() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_assignable_4() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_assignable_5() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_assignable_6() {
    const _: () = assert!(true, "( std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_not_assignable_7() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_not_assignable_8() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_not_assignable_9() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_not_assignable_10() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_not_assignable_11() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
pub unsafe fn test_is_not_assignable_12() {
    const _: () = assert!(true, "(!std::is_assignable<T, U>::value)");;
    const _: () = assert!(true, "!std::is_assignable_v<T, U>");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct E {}
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
    (unsafe { test_is_assignable_0() });
    (unsafe { test_is_assignable_1() });
    (unsafe { test_is_assignable_2() });
    (unsafe { test_is_assignable_3() });
    (unsafe { test_is_assignable_4() });
    (unsafe { test_is_assignable_5() });
    (unsafe { test_is_not_assignable_7() });
    (unsafe { test_is_not_assignable_8() });
    (unsafe { test_is_not_assignable_9() });
    (unsafe { test_is_not_assignable_10() });
    (unsafe { test_is_not_assignable_11() });
    (unsafe { test_is_not_assignable_12() });
    (unsafe { test_is_assignable_6() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
