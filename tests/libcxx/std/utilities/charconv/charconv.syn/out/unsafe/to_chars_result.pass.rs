extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    let mut lhs: () = unimplemented!();
    let mut rhs: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut __decomp_1: () = unimplemented!();
    const _: () = assert!(true, "std::is_same_v<decltype(ptr), char*>");;
    unimplemented!();
    const _: () = assert!(true, "std::is_same_v<decltype(ec), std::errc>");;
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
    const _: () = assert!(true, "std::equality_comparable<std::to_chars_result>");;
    const _: () = assert!(!(false), "!std::totally_ordered<std::to_chars_result>");;
    const _: () = assert!(!(false), "!std::three_way_comparable<std::to_chars_result>");;
    unimplemented!();
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
