extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_type_0() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1.clone();
    let mut cii: () = unimplemented!();
    assert!(ii1 == ii2);
    assert!(ii1 == ii4);
    assert!(ii1 == cii);
    assert!(!(ii1 != ii2));
    assert!(!(ii1 != cii));
    let mut v: libc::c_char = (0 as libc::c_char);
    let mut c: () = unimplemented!();
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = ii1.partial_cmp(&ii2).unwrap();
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = unimplemented!();
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_type_1() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1.clone();
    let mut cii: () = unimplemented!();
    assert!(ii1 == ii2);
    assert!(ii1 == ii4);
    assert!(ii1 == cii);
    assert!(!(ii1 != ii2));
    assert!(!(ii1 != cii));
    let mut v: i32 = 0_i32;
    let mut c: () = unimplemented!();
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = ii1.partial_cmp(&ii2).unwrap();
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = unimplemented!();
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_type_2() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1.clone();
    let mut cii: () = unimplemented!();
    assert!(ii1 == ii2);
    assert!(ii1 == ii4);
    assert!(ii1 == cii);
    assert!(!(ii1 != ii2));
    assert!(!(ii1 != cii));
    let mut v: Vec<libc::c_char> = vec![libc::c_char::default()];
    let mut c: () = unimplemented!();
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() == unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = ii1.partial_cmp(&ii2).unwrap();
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = unimplemented!();
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_3() -> bool {
    (unsafe { test_type_0() });
    (unsafe { test_type_1() });
    (unsafe { test_type_2() });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_3() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
