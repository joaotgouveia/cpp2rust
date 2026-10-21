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
    let mut a: () = 10_();
    let mut b: () = 20_();
    let mut pa: *mut () = (&mut a as *mut ());
    let mut pb: *mut () = (&mut b as *mut ());
    let mut ppa: *mut *mut () = (&mut pa as *mut *mut ());
    let mut ppb: *mut *mut () = (&mut pb as *mut *mut ());
    let mut r1: () = (*unimplemented!());
    let mut r2: () = (*unimplemented!());
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
