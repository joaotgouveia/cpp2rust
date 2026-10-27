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
pub struct test_macros_detail_is_same_std_plus_void___std_plus_void__ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_minus_void___std_minus_void__ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_multiplies_void___std_multiplies_void__ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_divides_void___std_divides_void__ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_modulus_void___std_modulus_void__ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_negate_void___std_negate_void__ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_bit_and_void___std_bit_and_void__ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_bit_not_void___std_bit_not_void__ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_bit_or_void___std_bit_or_void__ {}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_bit_xor_void___std_bit_xor_void__ {}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_equal_to_void___std_equal_to_void__ {}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_not_equal_to_void___std_not_equal_to_void__ {}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_less_void___std_less_void__ {}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_less_equal_void___std_less_equal_void__ {}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_greater_equal_void___std_greater_equal_void__ {}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_greater_void___std_greater_void__ {}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_logical_and_void___std_logical_and_void__ {}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_logical_not_void___std_logical_not_void__ {}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_logical_or_void___std_logical_or_void__ {}
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
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut f: () = unimplemented!();
        const _: () = assert!(true, "");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
