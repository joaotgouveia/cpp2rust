extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_is_trivially_constructible_0() {
    const _: () = assert!(true, "( std::is_trivially_constructible<T>::value)");;
    const _: () = assert!(true, "( std::is_trivially_constructible_v<T>)");;
}
pub unsafe fn test_is_trivially_constructible_1() {
    const _: () = assert!(true, "( std::is_trivially_constructible<T, A0>::value)");;
    const _: () = assert!(true, "( std::is_trivially_constructible_v<T, A0>)");;
}
pub unsafe fn test_is_not_trivially_constructible_2() {
    const _: () = assert!(true, "(!std::is_trivially_constructible<T>::value)");;
    const _: () = assert!(true, "(!std::is_trivially_constructible_v<T>)");;
}
pub unsafe fn test_is_not_trivially_constructible_3() {
    const _: () = assert!(true, "(!std::is_trivially_constructible<T, A0>::value)");;
    const _: () = assert!(true, "(!std::is_trivially_constructible_v<T, A0>)");;
}
pub unsafe fn test_is_not_trivially_constructible_4() {
    const _: () = assert!(true, "(!std::is_trivially_constructible<T, A0, A1>::value)");;
    const _: () = assert!(true, "(!std::is_trivially_constructible_v<T, A0, A1>)");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
impl A {}
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
    (unsafe { test_is_trivially_constructible_0() });
    (unsafe { test_is_trivially_constructible_1() });
    (unsafe { test_is_not_trivially_constructible_3() });
    (unsafe { test_is_not_trivially_constructible_4() });
    (unsafe { test_is_not_trivially_constructible_2() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
