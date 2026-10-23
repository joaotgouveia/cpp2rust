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
    let mut x: () = -1_();
    unimplemented!();
    let mut v1: () = ((x) & (1_()));
    let mut v2: () = ((x) & (1_()));
    let mut p: *mut () = (&mut x as *mut ());
    let mut b: () = (((*p) & (255_())) != 0);
    let mut a: () = ((*p) & (255_()));
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
