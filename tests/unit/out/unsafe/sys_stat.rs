extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_stat_0() {
    let mut path: *const () = (c"cpp2rust_stat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut st: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_fstat_1() {
    let mut path: *const () = (c"cpp2rust_fstat_test.tmp".as_ptr().cast_mut()).cast_const();
    let mut fp: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fd: () = unimplemented!();
    let mut st: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_st_mtime_access_2() {
    let mut st: () = unimplemented!();
    let mut sp: *mut () = (&mut st as *mut ());
    unimplemented!() = 5_();
    unimplemented!();
    unimplemented!() = 6_();
    unimplemented!();
    unimplemented!() += 2_();
    unimplemented!();
    unimplemented!() -= 1_();
    unimplemented!();
    unimplemented!().postfix_inc();
    unimplemented!().prefix_inc();
    unimplemented!();
    let mut mtime: *mut () = (&mut unimplemented!() as *mut ());
    (*mtime) = 10_();
    unimplemented!();
    let mut mtime_via_ptr: *mut () = (&mut unimplemented!() as *mut ());
    (*mtime_via_ptr) = 11_();
    unimplemented!();
    let mut copy: () = unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test_stat_0() });
    (unsafe { test_fstat_1() });
    (unsafe { test_st_mtime_access_2() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
