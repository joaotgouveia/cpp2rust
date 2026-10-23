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
pub struct test_macros_detail_is_same_std_chrono_time_point_std_chrono_local_t__std_chrono_duration_long_long__std_ratio_1__1_____std_chrono_time_point_std_chrono_local_t__std_chrono_duration_long_long__std_ratio_1__1____
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_time_point_std_chrono_local_t__std_chrono_duration_int__std_ratio_86400__1_____std_chrono_time_point_std_chrono_local_t__std_chrono_duration_int__std_ratio_86400__1____
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_time_point_std_chrono_local_t__std_chrono_duration_long__std_ratio_60__1_____std_chrono_time_point_std_chrono_local_t__std_chrono_duration_long__std_ratio_60__1____
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_int__std_ratio_86400__1____std_chrono_duration_int__std_ratio_86400__1___
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long_long__std_ratio_1__1____std_chrono_duration_long_long__std_ratio_1__1___
{}
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
    let January: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut d0: () = unimplemented!();
    let mut d1: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    let mut s0 : () = std_chrono_time_point_std_chrono_local_t__std_chrono_duration_long_long__std_ratio_1__1___ :: new_1 ( { & d0 } , )  ;
    let mut s1 : () = std_chrono_time_point_std_chrono_local_t__std_chrono_duration_long_long__std_ratio_1__1___ :: new_1 ( { & d1 } , )  ;
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
