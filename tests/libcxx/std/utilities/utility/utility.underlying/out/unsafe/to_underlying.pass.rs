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
pub struct test_macros_detail_is_same_int__int_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_short__unsigned_short_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long_long__long_long_ {}
pub type e_default = ();
pub const e_default_a: e_default = 0;
pub const e_default_b: e_default = 1;
pub const e_default_c: e_default = 2;
pub type e_ushort = ();
pub const e_ushort_d: e_ushort = 10;
pub const e_ushort_e: e_ushort = 25;
pub const e_ushort_f: e_ushort = 50;
pub type e_longlong = ();
pub const e_longlong_low: e_longlong = -9223372036854775808;
pub const e_longlong_high: e_longlong = 9223372036854775807;
pub type e_non_class = ();
pub const e_non_class_enum_a: e_non_class = 10;
pub const e_non_class_enum_b: e_non_class = 11;
pub const e_non_class_enum_c: e_non_class = 12;
pub type e_int = ();
pub const e_int_enum_min: e_int = -2147483648;
pub const e_int_enum_max: e_int = 2147483647;
pub type e_bool = ();
pub const e_bool_f: e_bool = 0;
pub const e_bool_t: e_bool = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct WithBitfieldEnums {
    pub e1: e_default,
    pub e2: e_ushort,
    pub e3: e_bool,
}
pub unsafe fn test_3() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut bf: WithBitfieldEnums = <WithBitfieldEnums>::default();
    bf.e1 = ((3_()) as e_default);
    bf.e2 = e_ushort_e;
    bf.e3 = e_bool_t;
    unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
