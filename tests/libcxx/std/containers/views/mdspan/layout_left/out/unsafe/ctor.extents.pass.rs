extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_construction_0(mut e: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
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
pub unsafe fn test_construction_1(mut e: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
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
pub unsafe fn test_construction_2(mut e: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
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
pub unsafe fn test_construction_3(mut e: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
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
pub unsafe fn test_construction_4(mut e: ()) {
    const _: () = assert!(true, "");;
    let mut m: () = unimplemented!();
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
    (unsafe { test_construction_0(unimplemented!()) });
    (unsafe { test_construction_1(unimplemented!()) });
    (unsafe { test_construction_2(unimplemented!()) });
    (unsafe { test_construction_3(unimplemented!()) });
    (unsafe { test_construction_4(unimplemented!()) });
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
