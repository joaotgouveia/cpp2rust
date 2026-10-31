extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut testcases_0: std::cell::LazyCell<[(); 153]> = std::cell::LazyCell::new(|| unsafe {
    [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ]
});
pub type anon_1 = u32;
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub unsafe fn classify_2(mut x: f64) -> i32 {
    if ((x) == (0_f64)) {
        return (anon_1_zero as i32);
    }
    if unimplemented!() {
        return (anon_1_inf as i32);
    }
    if unimplemented!() {
        return (anon_1_NaN as i32);
    }
    return (anon_1_non_zero as i32);
}
pub unsafe fn is_about_3(mut x: f32, mut y: f32) {
    assert!(((unimplemented!() as f64) < (1e-06_f64)));
}
pub unsafe fn is_about_4(mut x: f64, mut y: f64) {
    assert!(((unimplemented!()) < (1e-14_f64)));
}
pub unsafe fn is_about_5(mut x: f64, mut y: f64) {
    assert!(((unimplemented!()) < (1e-14_f64)));
}
pub unsafe fn test_6(mut x: i32, mut _a1: Option<*mut ::libc::c_void>) {
    let mut _a1: *mut ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_7(mut x: u32, mut _a1: Option<*mut ::libc::c_void>) {
    let mut _a1: *mut ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_8(mut x: i64, mut _a1: Option<*mut ::libc::c_void>) {
    let mut _a1: *mut ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_9(mut x: f32, mut _a1: Option<*mut ::libc::c_void>) {
    let mut _a1: *mut ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_10(mut x: f64, mut _a1: Option<*mut ::libc::c_void>) {
    let mut _a1: *mut ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_11(mut x: f64, mut _a1: Option<*mut ::libc::c_void>) {
    let mut _a1: *mut ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_12() {
    (unsafe { test_9(0_f32, None) });
    (unsafe { test_9(1_f32, None) });
    (unsafe { test_9(10_f32, None) });
}
pub unsafe fn test_13() {
    (unsafe { test_10(0_f64, None) });
    (unsafe { test_10(1_f64, None) });
    (unsafe { test_10(10_f64, None) });
}
pub unsafe fn test_14() {
    (unsafe { test_11(0_f64, None) });
    (unsafe { test_11(1_f64, None) });
    (unsafe { test_11(10_f64, None) });
}
pub unsafe fn test_15() {
    (unsafe { test_6(0, None) });
    (unsafe { test_6(1, None) });
    (unsafe { test_6(10, None) });
}
pub unsafe fn test_16() {
    (unsafe { test_7(0_u32, None) });
    (unsafe { test_7(1_u32, None) });
    (unsafe { test_7(10_u32, None) });
}
pub unsafe fn test_17() {
    (unsafe { test_8(0_i64, None) });
    (unsafe { test_8(1_i64, None) });
    (unsafe { test_8(10_i64, None) });
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
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
