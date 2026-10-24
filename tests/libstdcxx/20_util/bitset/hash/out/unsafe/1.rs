extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut b0: () = unimplemented!();
    let mut h0: () = unimplemented!();
    unimplemented!();
    let mut b1: () = unimplemented!();
    let mut h1: () = unimplemented!();
    (unsafe { h1(&b1) });
    let mut b2: () = unimplemented!();
    let mut h2: () = unimplemented!();
    (unsafe { h2(&b2) });
    let mut b3: () = unimplemented!();
    let mut h3: () = unimplemented!();
    (unsafe { h3(&b3) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_0() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
