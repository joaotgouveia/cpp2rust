extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn out_param_0(mut out: Option<*mut ()>) {
    let mut out: *mut () = out.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    if !(out).is_null() {
        (*out) = 4_();
    }
}
pub unsafe fn parse_1(mut v: (), mut idx: Option<*mut ()>) -> () {
    let mut idx: *mut () = idx.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    if !(idx).is_null() {
        (*idx) = 3_();
    }
    return v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut v2: () = 0_();
    (unsafe { out_param_0(Some((&mut v2 as *mut ()))) });
    unimplemented!();
    let mut pidx: () = 0_();
    unimplemented!();
    unimplemented!();
    let mut sv: () = 7_();
    let mut sp: *mut () = (&mut sv as *mut ());
    let mut spp: *mut *mut () = (&mut sp as *mut *mut ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
