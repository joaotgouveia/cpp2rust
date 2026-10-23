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
    let mut x: () = unimplemented!();
    unimplemented!();
    let test: [(); 5] = std::mem::transmute(*b"test\0");
    unimplemented!();
    unimplemented!();
    let mut best: [(); 5] = std::mem::transmute(*b"best\0");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut w: () = unimplemented!();
    unimplemented!();
    let west: [(); 5] = [119 as (), 101 as (), 115 as (), 116 as (), 0 as ()];
    unimplemented!();
    unimplemented!();
    let mut rest: [(); 5] = [114 as (), 101 as (), 115 as (), 116 as (), 0 as ()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
