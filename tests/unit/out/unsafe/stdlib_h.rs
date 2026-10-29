extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_setenv_getenv_0() {
    unimplemented!();
    let mut v: *const () = (unimplemented!()).cast_const();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    v = (unimplemented!()).cast_const();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_realpath_1() {
    let mut buf: [(); 4096] = [unimplemented!(); 4096];
    unimplemented!();
    unimplemented!();
    let mut p: *mut () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = 0_();
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
    (unsafe { test_setenv_getenv_0() });
    (unsafe { test_realpath_1() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
