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
pub struct TriviallyCopyable {
    pub i_: (),
}
impl TriviallyCopyable {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
}
pub unsafe fn test_0(mut t: TriviallyCopyable) {
    let mut t0: () = unimplemented!();
}
pub unsafe fn test_1(mut t: ()) {
    let mut t0: () = unimplemented!();
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
    (unsafe { test_0(TriviallyCopyable::new({ 42_() })) });
    (unsafe { test_1(unimplemented!()) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
