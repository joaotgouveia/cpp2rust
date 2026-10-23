extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_1() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_2() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_3() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_4() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_5() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_6() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_7() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_8() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_9() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_10() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_11() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_12() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_13() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_14() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_15() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_16() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_17() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_18() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_19() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_20() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_21() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_22() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_23() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_24() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_25() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_26() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_27() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_28() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_29() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_30() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_31() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
}
pub unsafe fn test_32() {
    let mut p: *mut () = std::ptr::null_mut();
    (&(p));
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
    (unsafe { test_9() });
    (unsafe { test_10() });
    (unsafe { test_11() });
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    (unsafe { test_20() });
    (unsafe { test_21() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
    (unsafe { test_31() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_4() });
    (unsafe { test_6() });
    (unsafe { test_8() });
    (unsafe { test_10() });
    (unsafe { test_12() });
    (unsafe { test_14() });
    (unsafe { test_16() });
    (unsafe { test_18() });
    (unsafe { test_20() });
    (unsafe { test_22() });
    (unsafe { test_24() });
    (unsafe { test_26() });
    (unsafe { test_5() });
    (unsafe { test_7() });
    (unsafe { test_9() });
    (unsafe { test_11() });
    (unsafe { test_13() });
    (unsafe { test_15() });
    (unsafe { test_17() });
    (unsafe { test_19() });
    (unsafe { test_21() });
    (unsafe { test_23() });
    (unsafe { test_25() });
    (unsafe { test_27() });
    (unsafe { test_32() });
    (unsafe { test_32() });
    (unsafe { test_32() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
