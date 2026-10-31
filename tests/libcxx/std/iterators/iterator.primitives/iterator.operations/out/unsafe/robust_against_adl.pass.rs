extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct Intable_Holder_Incomplete__ {}
impl Intable_Holder_Incomplete__ {
    pub unsafe fn to_i32(&self) -> i32 {
        return 1;
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
    let mut a: [*mut Holder_Incomplete_; 2] = [std::ptr::null_mut(), std::ptr::null_mut()];
    let mut p: *mut *mut Holder_Incomplete_ = a.as_mut_ptr();
    p = libcc2rs::next_unsafe(p, 1_isize);
    p = libcc2rs::prev_unsafe(p, 1_isize);
    p = libcc2rs::next_unsafe(p, 2_isize);
    p = libcc2rs::prev_unsafe(p, 2_isize);
    unimplemented!();
    &(unimplemented!());
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct Holder_Incomplete_;
pub unsafe fn __cpp2rust_init_globals() {}
