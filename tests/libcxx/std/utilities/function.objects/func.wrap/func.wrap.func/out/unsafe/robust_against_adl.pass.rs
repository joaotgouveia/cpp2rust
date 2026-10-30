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
pub struct Callable_Holder_Incomplete__ {}
impl Callable_Holder_Incomplete__ {
    pub unsafe fn operator_call(&self) {}
}
pub unsafe fn no_args_0() -> *mut Holder_Incomplete_ {
    return std::ptr::null_mut();
}
pub unsafe fn one_arg_1(mut p: *mut Holder_Incomplete_) -> *mut Holder_Incomplete_ {
    return p;
}
pub unsafe fn two_args_2(
    mut p: *mut Holder_Incomplete_,
    mut _a1: *mut Holder_Incomplete_,
) -> *mut Holder_Incomplete_ {
    return p;
}
pub unsafe fn three_args_3(
    mut p: *mut Holder_Incomplete_,
    mut _a1: *mut Holder_Incomplete_,
    mut _a2: *mut Holder_Incomplete_,
) -> *mut Holder_Incomplete_ {
    return p;
}
pub unsafe fn four_args_4(
    mut p: *mut Holder_Incomplete_,
    mut _a1: *mut Holder_Incomplete_,
    mut _a2: *mut Holder_Incomplete_,
    mut _a3: *mut Holder_Incomplete_,
) -> *mut Holder_Incomplete_ {
    return p;
}
pub unsafe fn one_arg_void_5(mut _a0: *mut Holder_Incomplete_) {}
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
    let mut x: *mut Holder_Incomplete_ = std::ptr::null_mut();
    let mut f: () = unimplemented!();
    unimplemented!();
    let mut g: () = unimplemented!();
    unimplemented!();
    let mut h: () = unimplemented!();
    unimplemented!();
    let mut i: () = unimplemented!();
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct Holder_Incomplete_;
pub unsafe fn __cpp2rust_init_globals() {}
