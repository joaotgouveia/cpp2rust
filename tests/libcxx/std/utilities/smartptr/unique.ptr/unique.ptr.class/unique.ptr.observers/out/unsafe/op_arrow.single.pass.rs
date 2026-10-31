extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct A {
    pub i_: i32,
}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self { i_: 7 };
        this
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub unsafe fn test_0() -> bool {
    let mut p: Option<Box<A>> = Some(Box::from_raw((Box::leak(Box::new(A::new())) as *mut A)));
    assert!(
        (((*p
            .as_deref()
            .map_or(::std::ptr::null_mut(), |v| v as *const A as *mut A))
        .i_) == (7))
    );
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
