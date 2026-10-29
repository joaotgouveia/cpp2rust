extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int___const_std_ranges_empty_view_int__
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int__ref__const_std_ranges_empty_view_int__ref_
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_int___const_std_ranges_empty_view_const_int__
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_int__ref__const_std_ranges_empty_view_const_int__ref_
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int_ptr___const_std_ranges_empty_view_int_ptr__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_int_ptr__ref__const_std_ranges_empty_view_int_ptr__ref_
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_Empty___const_std_ranges_empty_view_Empty__
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_Empty__ref__const_std_ranges_empty_view_Empty__ref_
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_Empty___const_std_ranges_empty_view_const_Empty__
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_const_Empty__ref__const_std_ranges_empty_view_const_Empty__ref_
{}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_BigType___const_std_ranges_empty_view_BigType__
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_ranges_empty_view_BigType__ref__const_std_ranges_empty_view_BigType__ref_
{}
pub unsafe fn testType_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut v: () = unimplemented!().clone();
    unimplemented!();
}
pub unsafe fn testType_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut v: () = unimplemented!().clone();
    unimplemented!();
}
pub unsafe fn testType_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut v: () = unimplemented!().clone();
    unimplemented!();
}
pub unsafe fn testType_15() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut v: () = unimplemented!().clone();
    unimplemented!();
}
pub unsafe fn testType_16() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut v: () = unimplemented!().clone();
    unimplemented!();
}
pub unsafe fn testType_17() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut v: () = unimplemented!().clone();
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
pub unsafe fn test_18() -> () {
    (unsafe { testType_12() });
    (unsafe { testType_13() });
    (unsafe { testType_14() });
    (unsafe { testType_15() });
    (unsafe { testType_16() });
    (unsafe { testType_17() });
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
    (unsafe { test_18() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
