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
pub struct A {}
pub static mut iArr1_0: std::cell::LazyCell<[i32; 10]> =
    std::cell::LazyCell::new(|| unsafe { [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] });
pub static mut iArr2_1: std::cell::LazyCell<[i32; 10]> =
    std::cell::LazyCell::new(|| unsafe { [10, 11, 12, 13, 14, 15, 16, 17, 18, 19] });

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
    const _: () = assert!(true, "noexcept(std::span<int>().empty())");;
    const _: () = assert!(true, "noexcept(std::span<int, 0>().empty())");;
    const _: () = assert!(true, "std::span<int>().empty()");;
    const _: () = assert!(true, "std::span<long>().empty()");;
    const _: () = assert!(true, "std::span<double>().empty()");;
    const _: () = assert!(true, "std::span<A>().empty()");;
    const _: () = assert!(true, "std::span<std::string>().empty()");;
    const _: () = assert!(true, "std::span<int, 0>().empty()");;
    const _: () = assert!(true, "std::span<long, 0>().empty()");;
    const _: () = assert!(true, "std::span<double, 0>().empty()");;
    const _: () = assert!(true, "std::span<A, 0>().empty()");;
    const _: () = assert!(true, "std::span<std::string, 0>().empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 1).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 2).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 3).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 4).empty()");;
    const _: () = assert!(true, "!std::span<const int>(iArr1, 5).empty()");;
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!((unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    assert!(!(unimplemented!()));
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    assert!((unimplemented!()));
    assert!(!(unimplemented!()));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_0);
    std::cell::LazyCell::force(&*&raw const iArr2_1);
}
