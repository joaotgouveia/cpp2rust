extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_required_alignment_0() {
    let required_alignment: usize = unimplemented!();
    assert!(((required_alignment) >= (::std::mem::align_of::<i32>())));
}
pub unsafe fn check_required_alignment_1() {
    let required_alignment: usize = unimplemented!();
    assert!(((required_alignment) >= (::std::mem::align_of::<f32>())));
}
pub unsafe fn check_required_alignment_2() {
    let required_alignment: usize = unimplemented!();
    assert!(((required_alignment) >= (::std::mem::align_of::<*mut i32>())));
}
pub unsafe fn check_required_alignment_3() {
    let required_alignment: usize = unimplemented!();
    assert!(((required_alignment) >= (::std::mem::align_of::<Empty_4>())));
}
pub unsafe fn check_required_alignment_5() {
    let required_alignment: usize = unimplemented!();
    assert!(((required_alignment) >= (::std::mem::align_of::<Trivial_6>())));
}
pub unsafe fn test_7() -> bool {
    (unsafe { check_required_alignment_0() });
    (unsafe { check_required_alignment_1() });
    (unsafe { check_required_alignment_2() });
    (unsafe { check_required_alignment_3() });
    (unsafe { check_required_alignment_5() });
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty_4 {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Trivial_6 {
    pub a: i32,
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
    (unsafe { test_7() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
