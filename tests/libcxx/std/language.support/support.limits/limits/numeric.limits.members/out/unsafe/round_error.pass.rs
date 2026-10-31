extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut expected: bool) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_1(mut expected: libc::c_char) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_2(mut expected: i8) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_3(mut expected: u8) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_4(mut expected: i32) {
    assert!(((unimplemented!() as i32) == (expected)));
    assert!(((unimplemented!() as i32) == (expected)));
    assert!(((unimplemented!() as i32) == (expected)));
    assert!(((unimplemented!() as i32) == (expected)));
}
pub unsafe fn test_5(mut expected: u8) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_6(mut expected: u16) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_7(mut expected: u32) {
    assert!(((unimplemented!() as u32) == (expected)));
    assert!(((unimplemented!() as u32) == (expected)));
    assert!(((unimplemented!() as u32) == (expected)));
    assert!(((unimplemented!() as u32) == (expected)));
}
pub unsafe fn test_8(mut expected: i16) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_9(mut expected: u16) {
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
    assert!(((unimplemented!() as i32) == (expected as i32)));
}
pub unsafe fn test_10(mut expected: i32) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_11(mut expected: u32) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_12(mut expected: i64) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_13(mut expected: u64) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_14(mut expected: i64) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_15(mut expected: u64) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_16(mut expected: i128) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_17(mut expected: u128) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_18(mut expected: f32) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_19(mut expected: f64) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
}
pub unsafe fn test_20(mut expected: f64) {
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
    assert!(((unimplemented!()) == (expected)));
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
    (unsafe { test_0(false) });
    (unsafe { test_1((0 as libc::c_char)) });
    (unsafe { test_2(0_i8) });
    (unsafe { test_3(0_u8) });
    (unsafe { test_4(0_i32) });
    (unsafe { test_5(0_u8) });
    (unsafe { test_6(0_u16) });
    (unsafe { test_7(0_u32) });
    (unsafe { test_8(0_i16) });
    (unsafe { test_9(0_u16) });
    (unsafe { test_10(0) });
    (unsafe { test_11(0_u32) });
    (unsafe { test_12(0_i64) });
    (unsafe { test_13(0_u64) });
    (unsafe { test_14(0_i64) });
    (unsafe { test_15(0_u64) });
    (unsafe { test_16(0_i128) });
    (unsafe { test_17(0_u128) });
    (unsafe { test_18((0.5_f64 as f32)) });
    (unsafe { test_19(0.5_f64) });
    (unsafe { test_20(0.5_f64) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
