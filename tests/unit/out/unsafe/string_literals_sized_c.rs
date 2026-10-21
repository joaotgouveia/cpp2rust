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
    let mut empty_buf: [(); 256] = [0 as libc::c_char; 256];
    unimplemented!();
    unimplemented!();
    let mut prefix_buf: [(); 32] =
        std::mem::transmute(*b"%\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut short_buf: [(); 16] = std::mem::transmute(*b"hi\0\0\0\0\0\0\0\0\0\0\0\0\0\0");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut exact_buf: [(); 6] = std::mem::transmute(*b"hello\0");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
