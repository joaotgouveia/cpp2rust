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
pub struct test_macros_detail_is_same_std_chrono_duration_long__std_ratio_3600__1____std_chrono_duration_long__std_ratio_3600__1___
{}
pub unsafe fn check_hours_1(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_hours_2(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_hours_3(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_hours_4(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_hours_5(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_hours_6(mut d: ()) -> i64 {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return (unimplemented!() as i64);
}
pub unsafe fn check_hours_7(mut d: ()) -> i64 {
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
    const _: () = assert!(true, "check_hours(std::chrono::minutes( 1)) == 0");;
    const _: () = assert!(true, "check_hours(std::chrono::minutes(-1)) == 0");;
    assert!(((unsafe { check_hours_2(unimplemented!(),) }) == (1_i64)));
    assert!(((unsafe { check_hours_2(unimplemented!(),) }) == (1_i64)));
    assert!(((unsafe { check_hours_1(unimplemented!(),) }) == (83_i64)));
    assert!(((unsafe { check_hours_1(unimplemented!(),) }) == (83_i64)));
    assert!(((unsafe { check_hours_3(unimplemented!(),) }) == (11_i64)));
    assert!(((unsafe { check_hours_3(unimplemented!(),) }) == (11_i64)));
    assert!(((unsafe { check_hours_4(unimplemented!(),) }) == (34_i64)));
    assert!(((unsafe { check_hours_4(unimplemented!(),) }) == (34_i64)));
    assert!(((unsafe { check_hours_5(unimplemented!(),) }) == (0_i64)));
    assert!(((unsafe { check_hours_5(unimplemented!(),) }) == (0_i64)));
    assert!(((unsafe { check_hours_6(unimplemented!(),) }) == (0_i64)));
    assert!(((unsafe { check_hours_6(unimplemented!(),) }) == (0_i64)));
    assert!(((unsafe { check_hours_7(unimplemented!(),) }) == (0_i64)));
    assert!(((unsafe { check_hours_7(unimplemented!(),) }) == (0_i64)));
    assert!(((unsafe { check_hours_7(unimplemented!(),) }) == (3_i64)));
    assert!(((unsafe { check_hours_7(unimplemented!(),) }) == (3_i64)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
