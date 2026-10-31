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
pub unsafe fn test_6(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (0 as f64)));
    let val: i32 = 0;
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_7(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (1 as f64)));
    let val: i32 = 1;
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_8(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (10 as f64)));
    let val: i32 = 10;
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_9(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (0 as f64)));
    let val: u32 = (0 as u32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_10(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (1 as f64)));
    let val: u32 = (1 as u32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_11(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (10 as f64)));
    let val: u32 = (10 as u32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_12(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (0 as f64)));
    let val: i64 = (0 as i64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_13(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (1 as f64)));
    let val: i64 = (1 as i64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_14(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::real(T(x))), double>::value)"
    );;
    assert!(((unimplemented!()) == (10 as f64)));
    let val: i64 = (10 as i64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_15(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (0 as f64)));
    let val: f32 = (0 as f32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_16(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (1 as f64)));
    let val: f32 = (1 as f32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_17(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (10 as f64)));
    let val: f32 = (10 as f32);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_18(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (0 as f64)));
    let val: f64 = (0 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_19(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (1 as f64)));
    let val: f64 = (1 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_20(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (10 as f64)));
    let val: f64 = (10 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_21(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (0 as f64)));
    let val: f64 = (0 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_22(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (1 as f64)));
    let val: f64 = (1 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_23(mut _a0: Option<*mut ::libc::c_void>) {
    let mut _a0: *mut ::libc::c_void = _a0.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    const _: () = assert!(true, "(std::is_same<decltype(std::real(T(x))), T>::value)");;
    assert!(((unimplemented!()) == (10 as f64)));
    let val: f64 = (10 as f64);
    const _: () = assert!(true, "std::real(val) == val");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.real() == x");;
}
pub unsafe fn test_24() {
    (unsafe { test_15(None) });
    (unsafe { test_16(None) });
    (unsafe { test_17(None) });
}
pub unsafe fn test_25() {
    (unsafe { test_18(None) });
    (unsafe { test_19(None) });
    (unsafe { test_20(None) });
}
pub unsafe fn test_26() {
    (unsafe { test_21(None) });
    (unsafe { test_22(None) });
    (unsafe { test_23(None) });
}
pub unsafe fn test_27() {
    (unsafe { test_6(None) });
    (unsafe { test_7(None) });
    (unsafe { test_8(None) });
}
pub unsafe fn test_28() {
    (unsafe { test_9(None) });
    (unsafe { test_10(None) });
    (unsafe { test_11(None) });
}
pub unsafe fn test_29() {
    (unsafe { test_12(None) });
    (unsafe { test_13(None) });
    (unsafe { test_14(None) });
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
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
