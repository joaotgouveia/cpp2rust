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
    let mut b1: () = 1_();
    let mut ushift1: () = 3_();
    let mut shl1: () = unimplemented!();
    unimplemented!();
    let mut ushift2: () = 2_();
    let mut shr1: () = unimplemented!();
    unimplemented!();
    let mut ushift3: () = 5_();
    unimplemented!();
    unimplemented!();
    let mut ushift4: () = 3_();
    unimplemented!();
    unimplemented!();
    let mut ishift: () = 1_();
    let mut shl2: () = unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
