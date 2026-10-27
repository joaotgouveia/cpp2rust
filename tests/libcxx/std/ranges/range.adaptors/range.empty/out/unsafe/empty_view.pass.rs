extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn testType_0() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let mut empty: () = unimplemented!();
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
}
pub unsafe fn testType_1() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let mut empty: () = unimplemented!();
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
}
pub unsafe fn testType_2() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let mut empty: () = unimplemented!();
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
}
pub unsafe fn testType_3() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let mut empty: () = unimplemented!();
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
}
pub unsafe fn testType_4() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let mut empty: () = unimplemented!();
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
}
pub unsafe fn testType_5() {
    const _: () = assert!(true, "std::ranges::range<std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::range<const std::ranges::empty_view<T>>");;
    const _: () = assert!(true, "std::ranges::view<std::ranges::empty_view<T>>");;
    let mut empty: () = unimplemented!();
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
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct BigType {
    pub buff: [(); 8],
}
pub unsafe fn test_6() -> () {
    const _: () = assert!(!(false), "!ValidEmptyView<int&>");;
    const _: () = assert!(!(false), "!ValidEmptyView<void>");;
    (unsafe { testType_0() });
    (unsafe { testType_1() });
    (unsafe { testType_2() });
    (unsafe { testType_3() });
    (unsafe { testType_4() });
    (unsafe { testType_5() });
    return true;
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
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
