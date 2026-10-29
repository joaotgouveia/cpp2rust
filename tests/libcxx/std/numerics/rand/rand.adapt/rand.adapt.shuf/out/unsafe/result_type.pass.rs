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
pub struct rand1_unsigned_long__0__10_ {
    x_: (),
}
impl rand1_unsigned_long__0__10_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rand1_unsigned_long_long__0__10_ {
    x_: (),
}
impl rand1_unsigned_long_long__0__10_ {}
pub unsafe fn test1_0() {
    const _: () = assert!(
        true,
        "(std::is_same<\n        std::shuffle_order_engine<rand1<unsigned long, 0, 10>, 16>::result_type,\n        unsigned long>::value)"
    );;
}
pub unsafe fn test2_1() {
    const _: () = assert!(
        true,
        "(std::is_same<\n        std::shuffle_order_engine<rand1<unsigned long long, 0, 10>, 16>::result_type,\n        unsigned long long>::value)"
    );;
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
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
