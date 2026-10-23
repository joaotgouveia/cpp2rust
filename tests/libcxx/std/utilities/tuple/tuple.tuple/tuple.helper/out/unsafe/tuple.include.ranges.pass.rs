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
pub struct SizedSentinel {}
const _: () = assert!(true, "std::sized_sentinel_for<SizedSentinel, Iterator>");
const _: () = assert!(
    !(false),
    "!std::sized_sentinel_for<UnsizedSentinel, Iterator>"
);
const _: () = assert!(true, "std::ranges::sized_range<ThreeElementRange>");
const _: () = assert!(true, "std::tuple_size<SizedRange>::value == 2");
const _: () = assert!(true, "std::tuple_size<UnsizedRange>::value == 2");
const _: () = assert!(true, "std::tuple_size<ThreeElementRange>::value == 2");
pub unsafe fn test_tuple_element_0() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
    return true;
}
pub unsafe fn test_tuple_element_1() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
    return true;
}
pub unsafe fn test_tuple_element_2() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
    return true;
}
pub unsafe fn test_tuple_element_3() -> () {
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, Range>::type, Expected>"
    );;
    const _: () = assert!(
        true,
        "std::same_as<typename std::tuple_element<I, const Range>::type, Expected>"
    );;
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
    const _: () = assert!(true, "test_tuple_element<0, SizedRange, Iterator>()");;
    const _: () = assert!(true, "test_tuple_element<1, SizedRange, SizedSentinel>()");;
    const _: () = assert!(true, "test_tuple_element<0, UnsizedRange, Iterator>()");;
    const _: () = assert!(
        true,
        "test_tuple_element<1, UnsizedRange, UnsizedSentinel>()"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
