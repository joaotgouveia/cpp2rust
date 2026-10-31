extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut b0: Vec<bool> = unimplemented!();
    let mut h0: () = unimplemented!();
    unimplemented!();
    let mut b1: Vec<bool> = vec![false; 10_usize];
    let mut h1: () = unimplemented!();
    unimplemented!();
    let mut b2: Vec<bool> = vec![false; 100_usize];
    let mut h2: () = unimplemented!();
    unimplemented!();
    let mut b3: Vec<bool> = vec![false; 1000_usize];
    let mut h3: () = unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
