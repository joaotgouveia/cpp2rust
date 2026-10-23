extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_construction_0() {
    const _: () = assert!(true, "");;
    let mut m: std_layout_right_mapping_std_extents_int__ =
        <std_layout_right_mapping_std_extents_int__>::default();
    let mut e: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        expected_size *= unimplemented!();
        r.postfix_inc();
    }
    unimplemented!();
}
pub unsafe fn test_construction_1() {
    const _: () = assert!(true, "");;
    let mut m: std_layout_right_mapping_std_extents_unsigned_int__18446744073709551615__ =
        <std_layout_right_mapping_std_extents_unsigned_int__18446744073709551615__>::default();
    let mut e: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        expected_size = (expected_size).wrapping_mul(unimplemented!());
        r.postfix_inc();
    }
    unimplemented!();
}
pub unsafe fn test_construction_2() {
    const _: () = assert!(true, "");;
    let mut m: std_layout_right_mapping_std_extents_unsigned_int__7__ =
        <std_layout_right_mapping_std_extents_unsigned_int__7__>::default();
    let mut e: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        expected_size = (expected_size).wrapping_mul(unimplemented!());
        r.postfix_inc();
    }
    unimplemented!();
}
pub unsafe fn test_construction_3() {
    const _: () = assert!(true, "");;
    let mut m: std_layout_right_mapping_std_extents_unsigned_int__7__8__ =
        <std_layout_right_mapping_std_extents_unsigned_int__7__8__>::default();
    let mut e: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        expected_size = (expected_size).wrapping_mul(unimplemented!());
        r.postfix_inc();
    }
    unimplemented!();
}
pub unsafe fn test_construction_4() {
    const _: () = assert!(true, "");;
    let mut m : std_layout_right_mapping_std_extents_long__18446744073709551615__8__18446744073709551615__18446744073709551615__ = <std_layout_right_mapping_std_extents_long__18446744073709551615__8__18446744073709551615__18446744073709551615__ >::default() ;
    let mut e: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut expected_size: () = 1_();
    let mut r: () = 0_();
    'loop_: while ((r) < (unimplemented!())) {
        expected_size *= unimplemented!();
        r.postfix_inc();
    }
    unimplemented!();
}
pub unsafe fn test_5() -> () {
    let D: () = (*std::cell::LazyCell::force_mut(&mut *&raw mut dynamic_extent_6));
    (unsafe { test_construction_0() });
    (unsafe { test_construction_1() });
    (unsafe { test_construction_2() });
    (unsafe { test_construction_3() });
    (unsafe { test_construction_4() });
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
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
