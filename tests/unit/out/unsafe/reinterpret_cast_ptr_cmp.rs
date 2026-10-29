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
    let mut i1: () = 42_();
    let mut ptr1: *mut () = (&mut i1 as *mut ());
    let mut ptr2: *mut () = ((&mut i1 as *mut ()) as *mut ());
    let mut vptr1: () = (ptr1 as ());
    let mut vptr2: () = (ptr2 as ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
