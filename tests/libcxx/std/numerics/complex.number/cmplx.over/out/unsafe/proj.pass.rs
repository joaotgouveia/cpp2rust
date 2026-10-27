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
pub unsafe fn test_8(mut x: (), mut _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<double> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_9(mut x: (), mut _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<double> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_10(mut x: (), mut _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<double> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_11(mut x: (), mut _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<T> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_12(mut x: (), mut _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<T> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_13(mut x: (), mut _a1: Option<()>) {
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::proj(x)), std::complex<T> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_14() {
    (unsafe { test_11(0_(), None) });
    (unsafe { test_11(1_(), None) });
    (unsafe { test_11(10_(), None) });
}
pub unsafe fn test_15() {
    (unsafe { test_12(0_(), None) });
    (unsafe { test_12(1_(), None) });
    (unsafe { test_12(10_(), None) });
}
pub unsafe fn test_16() {
    (unsafe { test_13(0_(), None) });
    (unsafe { test_13(1_(), None) });
    (unsafe { test_13(10_(), None) });
}
pub unsafe fn test_17() {
    (unsafe { test_8(0_(), None) });
    (unsafe { test_8(1_(), None) });
    (unsafe { test_8(10_(), None) });
}
pub unsafe fn test_18() {
    (unsafe { test_9(0_(), None) });
    (unsafe { test_9(1_(), None) });
    (unsafe { test_9(10_(), None) });
}
pub unsafe fn test_19() {
    (unsafe { test_10(0_(), None) });
    (unsafe { test_10(1_(), None) });
    (unsafe { test_10(10_(), None) });
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
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
