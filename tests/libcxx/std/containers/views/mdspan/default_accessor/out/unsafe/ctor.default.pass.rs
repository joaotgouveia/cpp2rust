extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct MinimalElementType {
    pub val: (),
}
impl MinimalElementType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
pub unsafe fn test_construction_0() {
    const _: () = assert!(true, "");;
    let mut acc: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_trivially_default_constructible_v<std::default_accessor<T>>"
    );;
}
pub unsafe fn test_construction_1() {
    const _: () = assert!(true, "");;
    let mut acc: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_trivially_default_constructible_v<std::default_accessor<T>>"
    );;
}
pub unsafe fn test_construction_2() {
    const _: () = assert!(true, "");;
    let mut acc: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_trivially_default_constructible_v<std::default_accessor<T>>"
    );;
}
pub unsafe fn test_construction_3() {
    const _: () = assert!(true, "");;
    let mut acc: () = unimplemented!();
    const _: () = assert!(
        true,
        "std::is_trivially_default_constructible_v<std::default_accessor<T>>"
    );;
}
pub unsafe fn test_4() -> () {
    (unsafe { test_construction_0() });
    (unsafe { test_construction_1() });
    (unsafe { test_construction_2() });
    (unsafe { test_construction_3() });
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
