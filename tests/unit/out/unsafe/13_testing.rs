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
    let mut a: () = 1_();
    let r: *mut () = &mut a;
    let mut p: *mut () = (&mut a as *mut ());
    (*r) = 2_();
    (*p) = 3_();
    let mut p2: *mut *mut () = (&mut p as *mut *mut ());
    (*(*p2)) = 3_();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
