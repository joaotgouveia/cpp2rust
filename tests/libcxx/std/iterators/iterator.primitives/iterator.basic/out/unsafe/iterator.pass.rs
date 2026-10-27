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
pub unsafe fn test2_0() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, std::ptrdiff_t>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<typename It::pointer, T*>::value)");;
    const _: () = assert!(true, "(std::is_same<typename It::reference, T&>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::forward_iterator_tag>::value)"
    );;
}
pub unsafe fn test3_1() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, short>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<typename It::pointer, T*>::value)");;
    const _: () = assert!(true, "(std::is_same<typename It::reference, T&>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::bidirectional_iterator_tag>::value)"
    );;
}
pub unsafe fn test4_2() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, int>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::pointer, const T*>::value)"
    );;
    const _: () = assert!(true, "(std::is_same<typename It::reference, T&>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::random_access_iterator_tag>::value)"
    );;
}
pub unsafe fn test5_3() {
    const _: () = assert!(true, "(std::is_same<typename It::value_type, T>::value)");;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::difference_type, long>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::pointer, const T*>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::reference, const T&>::value)"
    );;
    const _: () = assert!(
        true,
        "(std::is_same<typename It::iterator_category, std::input_iterator_tag>::value)"
    );;
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
    (unsafe { test2_0() });
    (unsafe { test3_1() });
    (unsafe { test4_2() });
    (unsafe { test5_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
