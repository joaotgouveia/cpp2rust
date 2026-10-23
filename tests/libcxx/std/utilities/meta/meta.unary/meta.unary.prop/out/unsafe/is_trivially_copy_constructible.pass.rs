extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_is_trivially_copy_constructible_0() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_is_trivially_copy_constructible_1() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_is_trivially_copy_constructible_2() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_is_trivially_copy_constructible_3() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_is_trivially_copy_constructible_4() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_is_trivially_copy_constructible_5() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_is_trivially_copy_constructible_6() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_is_trivially_copy_constructible_7() {
    const _: () = assert!(true, "std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible<const T>::value");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_has_not_trivial_copy_constructor_8() {
    const _: () = assert!(true, "!std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(
        true,
        "!std::is_trivially_copy_constructible<const T>::value"
    );;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_has_not_trivial_copy_constructor_9() {
    const _: () = assert!(true, "!std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(
        true,
        "!std::is_trivially_copy_constructible<const T>::value"
    );;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_has_not_trivial_copy_constructor_10() {
    const _: () = assert!(true, "!std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(
        true,
        "!std::is_trivially_copy_constructible<const T>::value"
    );;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<const T>");;
}
pub unsafe fn test_has_not_trivial_copy_constructor_11() {
    const _: () = assert!(true, "!std::is_trivially_copy_constructible<T>::value");;
    const _: () = assert!(
        true,
        "!std::is_trivially_copy_constructible<const T>::value"
    );;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<T>");;
    const _: () = assert!(true, "!std::is_trivially_copy_constructible_v<const T>");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotEmpty {}
#[repr(C)]
#[derive(Copy, Clone, FnPtrArg, VaArg)]
pub union Union {
    __empty: u8,
}
impl Default for Union {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct bit_zero {
    pub anon_12: (),
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
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
    (unsafe { test_has_not_trivial_copy_constructor_8() });
    (unsafe { test_has_not_trivial_copy_constructor_9() });
    (unsafe { test_has_not_trivial_copy_constructor_10() });
    (unsafe { test_has_not_trivial_copy_constructor_11() });
    (unsafe { test_is_trivially_copy_constructible_0() });
    (unsafe { test_is_trivially_copy_constructible_1() });
    (unsafe { test_is_trivially_copy_constructible_2() });
    (unsafe { test_is_trivially_copy_constructible_3() });
    (unsafe { test_is_trivially_copy_constructible_4() });
    (unsafe { test_is_trivially_copy_constructible_5() });
    (unsafe { test_is_trivially_copy_constructible_6() });
    (unsafe { test_is_trivially_copy_constructible_7() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
