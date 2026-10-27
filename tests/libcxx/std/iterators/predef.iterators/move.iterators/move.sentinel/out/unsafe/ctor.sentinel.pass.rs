extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    {
        const _: () = assert!(true, "!std::is_convertible_v<int, std::move_sentinel<int>>");;
        let mut m: () = unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "!std::is_convertible_v<int*, std::move_sentinel<int*>>"
        );;
        let mut i: () = 42_();
        let mut m: () = unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_convertible_v<S, std::move_sentinel<S>>");;
        let mut m: () = unimplemented!();
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct S_1 {
    pub i: (),
}
impl S_1 {
    pub unsafe fn new_1(mut j: ()) -> Self {
        let mut this = Self { i: j };
        this
    }
}
impl Default for S_1 {
    fn default() -> Self {
        unsafe { S_1 { i: 3_() } }
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
