extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut out: () = 0_();
    let mut x: () = 0_();
    let mut p1: *mut () = (&mut x as *mut ());
    (*p1) = 1_();
    out *= x;
    let mut p2: *mut *mut () = (&mut p1 as *mut *mut ());
    (*(*p2)) = 2_();
    out *= x;
    let mut p3: *mut *mut *mut () = (&mut p2 as *mut *mut *mut ());
    (*(*(*p3))) = 3_();
    out *= x;
    return out;
}
pub unsafe fn __cpp2rust_init_globals() {}
