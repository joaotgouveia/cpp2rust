extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test02_0() {
    let mut ccarray1: *const () = c"san francisco roof garden inspectors".as_ptr();
    let mut ccarray2: *const () = c"san francisco sunny-day park inspectors".as_ptr();
    let mut carray: [(); 50] = [unimplemented!(); 50];
    unimplemented!();
    let mut cv: () = (ccarray1 as ());
    let mut cv1: () = unimplemented!();
    let mut cc: *const () = std::ptr::null();
    let mut c: *mut () = std::ptr::null_mut();
    cv1 = unimplemented!();
    cc = unimplemented!();
    cc = unimplemented!();
    cc = unimplemented!();
    c = unimplemented!();
    cv1 = cv1;
    cc = cc;
    c = c;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test02_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
