extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe)]
pub struct A {}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn move_from(_a0: *mut A) -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct legacy {}
impl legacy {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for legacy {
    fn default() -> Self {
        unsafe { legacy::new() }
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
    let mut i: () = 0_();
    let ci: () = 0_();
    let mut l: legacy = legacy::new();
    let mut a: A = A::new();
    let ca: A = A::new();
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(i)), int&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(ci)), const int&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(a)), A&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(ca)), const A&&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::move_if_noexcept(l)), const legacy&>::value)"
    );;
    let i1: () = 23_();
    let i2: () = i1;
    const _: () = assert!(true, "i2 == 23");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
