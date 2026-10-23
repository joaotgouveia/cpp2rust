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
pub struct S {
    pub i: (),
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const S, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
pub unsafe fn operator_eq_1(x: *const S, y: *const S) -> () {
    return (((*x).i) == ((*y).i));
}
pub unsafe fn operator_eq_0(x: *const S, y: *const S) -> () {
    return (((*x).i) == ((*y).i));
}
pub unsafe fn test_2(t: *mut ()) {
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const                  (t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<               T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const          T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<volatile       T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const volatile T>(t))>::type>::value"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_3(t: *mut ()) {
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const                  (t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<               T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const          T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<volatile       T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const volatile T>(t))>::type>::value"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_4(t: *mut S) {
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const                  (t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<               T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const          T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<volatile       T>(t))>::type>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_const<typename std::remove_reference<decltype(std::as_const<const volatile T>(t))>::type>::value"
    );;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    let mut i: () = 3_();
    let mut d: () = 4_f64;
    let mut s: S = S { i: 2_() };
    (unsafe { test_2(&mut i) });
    (unsafe { test_3(&mut d) });
    (unsafe { test_4(&mut s) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
