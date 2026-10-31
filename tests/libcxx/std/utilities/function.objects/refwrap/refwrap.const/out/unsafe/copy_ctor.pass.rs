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
pub struct functor1 {}
pub unsafe fn test_0(t: *mut Option<unsafe fn()>) {
    let mut r: *mut Option<unsafe fn()> = t;
    let mut r2: *mut Option<unsafe fn()> = r;
    assert!(((r2) == (t)));
}
pub unsafe fn test_1(t: Option<unsafe fn()>) {
    let mut r: Option<unsafe fn()> = t;
    let mut r2: Option<unsafe fn()> = r;
    assert!(((r2) == (t)));
}
pub unsafe fn test_2(t: *mut functor1) {
    let mut r: *mut functor1 = t;
    let mut r2: *mut functor1 = r;
    assert!(((r2) == (t)));
}
pub unsafe fn test_3(t: *mut i32) {
    let mut r: *mut i32 = t;
    let mut r2: *mut i32 = r;
    assert!(((r2) == (t)));
}
pub unsafe fn test_4(t: *const i32) {
    let mut r: *const i32 = t;
    let mut r2: *const i32 = r;
    assert!(((r2) == (t)));
}
pub unsafe fn f_5() {}
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
    let mut fp: Option<unsafe fn()> = Some(f_5);
    (unsafe { test_0(&mut fp) });
    (unsafe { test_1(Some(f_5)) });
    let mut f1: functor1 = <functor1>::default();
    (unsafe { test_2(&mut f1) });
    let mut i: i32 = 0;
    (unsafe { test_3(&mut i) });
    let j: i32 = 0;
    (unsafe { test_4(&j) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
