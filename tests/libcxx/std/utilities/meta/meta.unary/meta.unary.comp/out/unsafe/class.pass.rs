extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_class_imp_0() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_imp_1() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_imp_2() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_imp_3() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_imp_4() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_imp_5() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_imp_6() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_imp_7() {
    const _: () = assert!(true, "!std::is_reference<T>::value");;
    const _: () = assert!(true, "!std::is_arithmetic<T>::value");;
    const _: () = assert!(true, "!std::is_fundamental<T>::value");;
    const _: () = assert!(true, "std::is_object<T>::value");;
    const _: () = assert!(true, "!std::is_scalar<T>::value");;
    const _: () = assert!(true, "std::is_compound<T>::value");;
    const _: () = assert!(true, "!std::is_member_pointer<T>::value");;
}
pub unsafe fn test_class_8() {
    (unsafe { test_class_imp_0() });
    (unsafe { test_class_imp_1() });
    (unsafe { test_class_imp_2() });
    (unsafe { test_class_imp_3() });
}
pub unsafe fn test_class_9() {
    (unsafe { test_class_imp_4() });
    (unsafe { test_class_imp_5() });
    (unsafe { test_class_imp_6() });
    (unsafe { test_class_imp_7() });
}
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
    (unsafe { test_class_8() });
    (unsafe { test_class_9() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
