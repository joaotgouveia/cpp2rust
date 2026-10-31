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
#[cpp_into(f32, to_f32_1_float)]
#[cpp_into(f64, to_f64_1_double)]
pub struct NastyConvertible {}
impl NastyConvertible {
    pub unsafe fn to_f32_1_float(&self) -> f32 {
        return 0_f32;
    }
    pub unsafe fn to_f64_1_double(&self) -> f64 {
        return 0_f64;
    }
    pub unsafe fn to_f64_1_long_double(&self) -> f64 {
        return 0_f64;
    }
}
pub unsafe fn test_0() {
    let mut nasty: NastyConvertible = <NastyConvertible>::default();
    let mut x: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f32)));
    assert!(((unimplemented!()) == (0_f32)));
}
pub unsafe fn test_1() {
    let mut nasty: NastyConvertible = <NastyConvertible>::default();
    let mut x: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (0_f64)));
}
pub unsafe fn test_2() {
    let mut nasty: NastyConvertible = <NastyConvertible>::default();
    let mut x: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
    assert!(((unimplemented!()) == (0_f64)));
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
    (unsafe { test_1() });
    (unsafe { test_2() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
