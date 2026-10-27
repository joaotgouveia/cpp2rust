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
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_short__unsigned_short_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long_long__unsigned_long_long_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_unsigned_int__const_unsigned_int_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned___int128__unsigned___int128_ {}
pub type Enum = ();
pub const Enum_zero: Enum = 0;
pub const Enum_one_: Enum = 1;
pub type BigEnum = ();
pub const BigEnum_bigzero: BigEnum = 0;
pub const BigEnum_big: BigEnum = 18446744073709551615;
pub type HugeEnum = ();
pub const HugeEnum_hugezero: HugeEnum = 0;
pub unsafe fn test_make_unsigned_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_10() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_12() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_14() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_15() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_16() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_17() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_18() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_19() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_20() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_21() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_22() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_23() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_make_unsigned_24() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
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
    (unsafe { test_make_unsigned_7() });
    (unsafe { test_make_unsigned_8() });
    (unsafe { test_make_unsigned_9() });
    (unsafe { test_make_unsigned_10() });
    (unsafe { test_make_unsigned_11() });
    (unsafe { test_make_unsigned_12() });
    (unsafe { test_make_unsigned_13() });
    (unsafe { test_make_unsigned_14() });
    (unsafe { test_make_unsigned_15() });
    (unsafe { test_make_unsigned_16() });
    (unsafe { test_make_unsigned_17() });
    (unsafe { test_make_unsigned_18() });
    (unsafe { test_make_unsigned_19() });
    (unsafe { test_make_unsigned_20() });
    (unsafe { test_make_unsigned_21() });
    (unsafe { test_make_unsigned_22() });
    (unsafe { test_make_unsigned_23() });
    (unsafe { test_make_unsigned_24() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
