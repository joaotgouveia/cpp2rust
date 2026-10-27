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
pub struct TestHelper_wchar_t__4_ {}
impl TestHelper_wchar_t__4_ {
    pub unsafe fn test() {
        {
            let mut c: () = unimplemented!();
            let mut r: () = (unsafe { c.max_length() });
            unimplemented!();
        }
        {
            let mut c: () = unimplemented!();
            let mut r: () = (unsafe { c.max_length() });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHelper_char16_t__2_ {}
impl TestHelper_char16_t__2_ {
    pub unsafe fn test() {
        {
            let mut c: () = unimplemented!();
            let mut r: () = (unsafe { c.max_length() });
            unimplemented!();
        }
        {
            let mut c: () = unimplemented!();
            let mut r: () = (unsafe { c.max_length() });
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestHelper_char32_t__4_ {}
impl TestHelper_char32_t__4_ {
    pub unsafe fn test() {
        {
            let mut c: () = unimplemented!();
            let mut r: () = (unsafe { c.max_length() });
            unimplemented!();
        }
        {
            let mut c: () = unimplemented!();
            let mut r: () = (unsafe { c.max_length() });
            unimplemented!();
        }
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
    (unsafe { TestHelper_wchar_t__4_::test() });
    (unsafe { TestHelper_char16_t__2_::test() });
    (unsafe { TestHelper_char32_t__4_::test() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
