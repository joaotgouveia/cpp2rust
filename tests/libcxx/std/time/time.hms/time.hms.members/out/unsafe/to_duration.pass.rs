extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1____std_chrono_duration_long__std_ratio_1__1___
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1000____std_chrono_duration_long__std_ratio_1__1000___
{}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1000000____std_chrono_duration_long__std_ratio_1__1000000___
{}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__1000000000____std_chrono_duration_long__std_ratio_1__1000000000___
{}
pub type anon_4 = u32;
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_1__10000____std_chrono_duration_long__std_ratio_1__10000___
{}
pub unsafe fn check_duration_5(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_duration_6(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_duration_7(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_duration_8(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_duration_9(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_duration_10(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_duration_11(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    const _: () = assert!(true, "check_duration(std::chrono::minutes( 1)) ==  60");;
    const _: () = assert!(true, "check_duration(std::chrono::minutes(-1)) == -60");;
    assert!(((unsafe { check_duration_6(unimplemented!(),) }) == (5000_i64)));
    assert!(((unsafe { check_duration_6(unimplemented!(),) }) == (-5000_i64)));
    assert!(((unsafe { check_duration_5(unimplemented!(),) }) == (300000_i64)));
    assert!(((unsafe { check_duration_5(unimplemented!(),) }) == (-300000_i64)));
    assert!(((unsafe { check_duration_7(unimplemented!(),) }) == (39600_i64)));
    assert!(((unsafe { check_duration_7(unimplemented!(),) }) == (-39600_i64)));
    assert!(((unsafe { check_duration_8(unimplemented!(),) }) == (123456789_i64)));
    assert!(((unsafe { check_duration_8(unimplemented!(),) }) == (-123456789_i64)));
    assert!(((unsafe { check_duration_9(unimplemented!(),) }) == (123456789_i64)));
    assert!(((unsafe { check_duration_9(unimplemented!(),) }) == (-123456789_i64)));
    assert!(((unsafe { check_duration_10(unimplemented!(),) }) == (123456789_i64)));
    assert!(((unsafe { check_duration_10(unimplemented!(),) }) == (-123456789_i64)));
    assert!(((unsafe { check_duration_11(unimplemented!(),) }) == (12096000_i64)));
    assert!(((unsafe { check_duration_11(unimplemented!(),) }) == (-12096000_i32 as i64)));
    assert!(((unsafe { check_duration_11(unimplemented!(),) }) == (120960000_i64)));
    assert!(((unsafe { check_duration_11(unimplemented!(),) }) == (-120960000_i32 as i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
