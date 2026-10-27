extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_is_same_0() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub unsafe fn test_is_same_1() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub unsafe fn test_is_same_2() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub unsafe fn test_is_same_3() {
    const _: () = assert!(true, "( std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(!std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "( std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(!std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "( std::is_same_v<const T, const U>)");;
}
pub unsafe fn test_is_same_ref_4() {
    const _: () = assert!(true, "(std::is_same<T, U>::value)");;
    const _: () = assert!(true, "(std::is_same<const T, U>::value)");;
    const _: () = assert!(true, "(std::is_same<T, const U>::value)");;
    const _: () = assert!(true, "(std::is_same<const T, const U>::value)");;
    const _: () = assert!(true, "(std::is_same_v<T, U>)");;
    const _: () = assert!(true, "(std::is_same_v<const T, U>)");;
    const _: () = assert!(true, "(std::is_same_v<T, const U>)");;
    const _: () = assert!(true, "(std::is_same_v<const T, const U>)");;
}
pub unsafe fn test_is_not_same_5() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub unsafe fn test_is_not_same_6() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub unsafe fn test_is_not_same_7() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub unsafe fn test_is_not_same_8() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
pub unsafe fn test_is_not_same_9() {
    const _: () = assert!(true, "(!std::is_same<T, U>::value)");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct OverloadTest_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Class {}
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
    (unsafe { test_is_same_0() });
    (unsafe { test_is_same_1() });
    (unsafe { test_is_same_2() });
    (unsafe { test_is_same_3() });
    (unsafe { test_is_same_ref_4() });
    (unsafe { test_is_not_same_5() });
    (unsafe { test_is_not_same_6() });
    (unsafe { test_is_not_same_7() });
    (unsafe { test_is_not_same_8() });
    (unsafe { test_is_not_same_9() });
    let mut t: OverloadTest_char_ = <OverloadTest_char_>::default();
    &(t);
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
