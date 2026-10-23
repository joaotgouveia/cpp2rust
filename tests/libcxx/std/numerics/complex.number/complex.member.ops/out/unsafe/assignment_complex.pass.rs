extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    c = (c3).clone();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_1() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_2() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_3() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_4() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    c = (c3).clone();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_5() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_6() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_7() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_8() -> () {
    let mut c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: () = unimplemented!();
    c = (c2).clone();
    unimplemented!();
    unimplemented!();
    let mut c3: () = unimplemented!();
    c = (c3).clone();
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
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_5() });
    (unsafe { test_6() });
    (unsafe { test_7() });
    (unsafe { test_8() });
    const _: () = assert!(true, "test<float, float>()");;
    const _: () = assert!(true, "test<float, double>()");;
    const _: () = assert!(true, "test<float, long double>()");;
    const _: () = assert!(true, "test<double, float>()");;
    const _: () = assert!(true, "test<double, double>()");;
    const _: () = assert!(true, "test<double, long double>()");;
    const _: () = assert!(true, "test<long double, float>()");;
    const _: () = assert!(true, "test<long double, double>()");;
    const _: () = assert!(true, "test<long double, long double>()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
