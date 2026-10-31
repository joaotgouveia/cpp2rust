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
unsafe fn main_0() -> i32 {
    let mut x: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    let test: [libc::c_char; 5] = std::mem::transmute(*b"test\0");
    unimplemented!();
    unimplemented!();
    let mut best: [libc::c_char; 5] = std::mem::transmute(*b"best\0");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut w: Vec<i32> = vec![i32::default()];
    unimplemented!();
    let west: [i32; 5] = [119 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32];
    unimplemented!();
    unimplemented!();
    let mut rest: [i32; 5] = [114 as i32, 101 as i32, 115 as i32, 116 as i32, 0 as i32];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
