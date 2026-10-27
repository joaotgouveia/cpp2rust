extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_enum_imp_0() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "!std::is_function<T>::value");;
}
pub unsafe fn test_enum_imp_1() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "!std::is_function<T>::value");;
}
pub unsafe fn test_enum_imp_2() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "!std::is_function<T>::value");;
}
pub unsafe fn test_enum_imp_3() {
    const _: () = assert!(true, "!std::is_void<T>::value");;
    const _: () = assert!(true, "!std::is_null_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_integral<T>::value");;
    const _: () = assert!(true, "!std::is_floating_point<T>::value");;
    const _: () = assert!(true, "!std::is_array<T>::value");;
    const _: () = assert!(true, "!std::is_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_lvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_rvalue_reference<T>::value");;
    const _: () = assert!(true, "!std::is_member_object_pointer<T>::value");;
    const _: () = assert!(true, "!std::is_member_function_pointer<T>::value");;
    const _: () = assert!(true, "std::is_enum<T>::value");;
    const _: () = assert!(true, "!std::is_union<T>::value");;
    const _: () = assert!(true, "!std::is_class<T>::value");;
    const _: () = assert!(true, "!std::is_function<T>::value");;
}
pub unsafe fn test_enum_4() {
    (unsafe { test_enum_imp_0() });
    (unsafe { test_enum_imp_1() });
    (unsafe { test_enum_imp_2() });
    (unsafe { test_enum_imp_3() });
}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one: Enum = 1;
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
    (unsafe { test_enum_4() });
    const _: () = assert!(true, "!std::is_enum<incomplete_type>::value");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
