extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_required_alignment_0() {
    let required_alignment: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn check_required_alignment_1() {
    let required_alignment: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn check_required_alignment_2() {
    let required_alignment: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn check_required_alignment_3() {
    let required_alignment: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn check_required_alignment_4() {
    let required_alignment: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_5() -> () {
    (unsafe { check_required_alignment_0() });
    (unsafe { check_required_alignment_1() });
    (unsafe { check_required_alignment_2() });
    (unsafe { check_required_alignment_3() });
    (unsafe { check_required_alignment_4() });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty_6 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trivial_7 {
    pub a: (),
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
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
