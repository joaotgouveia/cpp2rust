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
pub type anon_1 = ();
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub unsafe fn classify_2(mut x: ()) -> () {
    if ((x) == (0_())) {
        return (anon_1_zero as ());
    }
    if (unsafe { isinf_3(x) }) {
        return (anon_1_inf as ());
    }
    if (unsafe { isnan_4(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub unsafe fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn test_8(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 0_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_9(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 1_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_10(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 10_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_11(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 0_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_12(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 1_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_13(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 10_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_14(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 0_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_15(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 1_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_16(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::imag(T(x))), double>::value)"
    );;
    unimplemented!();
    let val: () = 10_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_17(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 0_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_18(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 1_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_19(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 10_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_20(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 0_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_21(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 1_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_22(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 10_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_23(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 0_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_24(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 1_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_25(mut _a0: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(true, "(std::is_same<decltype(std::imag(T(x))), T>::value)");;
    unimplemented!();
    let val: () = 10_();
    const _: () = assert!(true, "std::imag(val) == 0");;
    let t: () = unimplemented!();
    const _: () = assert!(true, "t.imag() == x");;
}
pub unsafe fn test_26() {
    (unsafe { test_17(None) });
    (unsafe { test_18(None) });
    (unsafe { test_19(None) });
}
pub unsafe fn test_27() {
    (unsafe { test_20(None) });
    (unsafe { test_21(None) });
    (unsafe { test_22(None) });
}
pub unsafe fn test_28() {
    (unsafe { test_23(None) });
    (unsafe { test_24(None) });
    (unsafe { test_25(None) });
}
pub unsafe fn test_29() {
    (unsafe { test_8(None) });
    (unsafe { test_9(None) });
    (unsafe { test_10(None) });
}
pub unsafe fn test_30() {
    (unsafe { test_11(None) });
    (unsafe { test_12(None) });
    (unsafe { test_13(None) });
}
pub unsafe fn test_31() {
    (unsafe { test_14(None) });
    (unsafe { test_15(None) });
    (unsafe { test_16(None) });
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
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
    (unsafe { test_31() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
