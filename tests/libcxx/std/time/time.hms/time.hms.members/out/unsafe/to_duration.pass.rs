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
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1____std_chrono_duration_long__std_ratio_1__1___
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1000____std_chrono_duration_long__std_ratio_1__1000___
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1000000____std_chrono_duration_long__std_ratio_1__1000000___
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1000000000____std_chrono_duration_long__std_ratio_1__1000000000___
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__10000____std_chrono_duration_long__std_ratio_1__10000___
{}
pub unsafe fn check_duration_5(mut d: ()) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub unsafe fn check_duration_6(mut d: ()) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub unsafe fn check_duration_7(mut d: ()) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub unsafe fn check_duration_8(mut d: ()) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub unsafe fn check_duration_9(mut d: ()) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub unsafe fn check_duration_10(mut d: ()) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub unsafe fn check_duration_11(mut d: ()) -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
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
    const _: () = assert!(true, "check_duration(std::chrono::minutes( 1)) ==  60");;
    const _: () = assert!(true, "check_duration(std::chrono::minutes(-1)) == -60");;
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
