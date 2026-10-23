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
pub struct test_macros_detail_is_same_unsigned_int__unsigned_int_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_layout_stride__std_layout_stride_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_signed_char__signed_char_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_char__unsigned_char_ {}
pub unsafe fn test_layout_mapping_requirements_6() {
    (unsafe { test_mapping_requirements_7(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_requirements_8() {
    (unsafe { test_mapping_requirements_9(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_requirements_10() {
    (unsafe { test_mapping_requirements_11(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_requirements_12() {
    (unsafe { test_mapping_requirements_13(unsafe { std::mem::zeroed::<()>() }) });
}
pub unsafe fn test_layout_mapping_stride_14() {
    (unsafe { test_layout_mapping_requirements_6() });
}
pub unsafe fn test_layout_mapping_stride_15() {
    (unsafe { test_layout_mapping_requirements_8() });
}
pub unsafe fn test_layout_mapping_stride_16() {
    (unsafe { test_layout_mapping_requirements_10() });
}
pub unsafe fn test_layout_mapping_stride_17() {
    (unsafe { test_layout_mapping_requirements_12() });
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
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_18));
    (unsafe { test_layout_mapping_stride_14() });
    (unsafe { test_layout_mapping_stride_15() });
    (unsafe { test_layout_mapping_stride_16() });
    (unsafe { test_layout_mapping_stride_17() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
