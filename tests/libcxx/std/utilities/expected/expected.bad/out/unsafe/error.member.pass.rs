extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(!(false), "!ErrorNoexcept<int>");
const _: () = assert!(true, "ErrorNoexcept<std::bad_expected_access<int>&>");
const _: () = assert!(true, "ErrorNoexcept<std::bad_expected_access<int> const&>");
const _: () = assert!(true, "ErrorNoexcept<std::bad_expected_access<int>&&>");
const _: () = assert!(true, "ErrorNoexcept<std::bad_expected_access<int> const&&>");
pub unsafe fn test_0() {
    {
        let mut e: () = unimplemented!();
        let i: *mut i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(i), int&>");;
        assert!(((*i) == (5)));
    }
    {
        let e: () = unimplemented!();
        let i: *const i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(i), const int&>");;
        assert!(((*i) == (5)));
    }
    {
        let mut e: () = unimplemented!();
        let i: *mut i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(i), int&&>");;
        assert!(((*i) == (5)));
    }
    {
        let e: () = unimplemented!();
        let i: *const i32 = unimplemented!();
        const _: () = assert!(true, "std::same_as<decltype(i), const int&&>");;
        assert!(((*i) == (5)));
    }
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
