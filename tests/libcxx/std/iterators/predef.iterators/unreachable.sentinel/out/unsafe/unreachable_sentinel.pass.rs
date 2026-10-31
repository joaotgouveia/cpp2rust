extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    const _: () = assert!(true, "std::is_empty_v<std::unreachable_sentinel_t>");;
    const _: () = assert!(true, "std::semiregular<std::unreachable_sentinel_t>");;
    const _: () = assert!(
        true,
        "std::same_as<decltype(std::unreachable_sentinel), const std::unreachable_sentinel_t>"
    );;
    let mut sentinel: () = unimplemented!();
    let mut i: i32 = 42;
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    let mut p: *mut i32 = std::ptr::null_mut();
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    const _: () = assert!(
        true,
        "weakly_equality_comparable_with<std::unreachable_sentinel_t, int>"
    );;
    const _: () = assert!(
        true,
        "weakly_equality_comparable_with<std::unreachable_sentinel_t, int*>"
    );;
    const _: () = assert!(
        !(false),
        "!weakly_equality_comparable_with<std::unreachable_sentinel_t, void*>"
    );;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
