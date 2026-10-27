extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_argument_type_std_function_int_____ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_argument_type_std_function_double__int__char___ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_argument_type_std_function_double__int__char__double___ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_first_argument_type_std_function_int_____ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_first_argument_type_std_function_double__int___ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_first_argument_type_std_function_double__int__char__double___ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_second_argument_type_std_function_int_____ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_second_argument_type_std_function_double__int___ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 0;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct has_second_argument_type_std_function_double__int__char__double___ {}
pub unsafe fn test_nullary_function_9() {
    const _: () = assert!(true, "(!has_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_first_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_second_argument_type<F>::value)");;
}
pub unsafe fn test_unary_function_10() {
    const _: () = assert!(true, "(!has_first_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_second_argument_type<F>::value)");;
}
pub unsafe fn test_binary_function_11() {
    const _: () = assert!(true, "(!has_argument_type<F>::value)");;
}
pub unsafe fn test_other_function_12() {
    const _: () = assert!(true, "(!has_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_first_argument_type<F>::value)");;
    const _: () = assert!(true, "(!has_second_argument_type<F>::value)");;
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
    (unsafe { test_nullary_function_9() });
    (unsafe { test_unary_function_10() });
    (unsafe { test_binary_function_11() });
    (unsafe { test_other_function_12() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
