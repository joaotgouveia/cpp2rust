extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_is_const_0() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
}
pub unsafe fn test_is_const_1() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
}
pub unsafe fn test_is_const_2() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
}
pub unsafe fn test_is_const_3() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
}
pub unsafe fn test_is_const_4() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
}
pub unsafe fn test_is_const_5() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
}
pub unsafe fn test_is_const_6() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
}
pub unsafe fn test_is_const_7() {
    const _: () = assert!(true, "!std::is_const<T>::value");;
    const _: () = assert!(true, "std::is_const<const T>::value");;
    const _: () = assert!(true, "!std::is_const<volatile T>::value");;
    const _: () = assert!(true, "std::is_const<const volatile T>::value");;
    const _: () = assert!(true, "!std::is_const_v<T>");;
    const _: () = assert!(true, "std::is_const_v<const T>");;
    const _: () = assert!(true, "!std::is_const_v<volatile T>");;
    const _: () = assert!(true, "std::is_const_v<const volatile T>");;
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
    (unsafe { test_is_const_0() });
    (unsafe { test_is_const_1() });
    (unsafe { test_is_const_2() });
    (unsafe { test_is_const_3() });
    (unsafe { test_is_const_4() });
    (unsafe { test_is_const_5() });
    (unsafe { test_is_const_6() });
    (unsafe { test_is_const_7() });
    const _: () = assert!(true, "!std::is_const<int&>::value");;
    const _: () = assert!(true, "!std::is_const<const int&>::value");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
