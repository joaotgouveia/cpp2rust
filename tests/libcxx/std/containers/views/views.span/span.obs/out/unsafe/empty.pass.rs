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
pub static mut iArr1_0: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [0_(), 1_(), 2_(), 3_(), 4_(), 5_(), 6_(), 7_(), 8_(), 9_()]
});
pub static mut iArr2_1: std::cell::LazyCell<[(); 10]> = std::cell::LazyCell::new(|| unsafe {
    [
        10_(),
        11_(),
        12_(),
        13_(),
        14_(),
        15_(),
        16_(),
        17_(),
        18_(),
        19_(),
    ]
});

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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const iArr1_0);
    std::cell::LazyCell::force(&*&raw const iArr2_1);
}
