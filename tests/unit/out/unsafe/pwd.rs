extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_getpwuid_0() {
    let mut pw: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        libc::printf(
            (c"%s\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            ((*pw).pw_name),
        )
    });
}
pub unsafe fn test_getpwuid_missing_1() {
    (*unimplemented!()) = 0_();
    let mut pw: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_getpwuid_r_2() {
    let mut pw: () = unimplemented!();
    let mut buf: [(); 4096] = [unimplemented!(); 4096];
    let mut result: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut pw2: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    (unsafe {
        libc::printf(
            (c"%s\n".as_ptr().cast_mut()).cast_const() as *const libc::c_char,
            (pw.pw_name),
        )
    });
}
pub unsafe fn test_getpwuid_r_erange_3() {
    let mut pw: () = unimplemented!();
    let mut tiny: [(); 1] = [unimplemented!(); 1];
    let mut result: *mut () = std::ptr::null_mut();
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_getpwuid_0() });
    (unsafe { test_getpwuid_missing_1() });
    (unsafe { test_getpwuid_r_2() });
    (unsafe { test_getpwuid_r_erange_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
