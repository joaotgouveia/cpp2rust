extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> () {
    return 0_();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = ((unsafe { foo_0() }) + (1_()));
    let mut y: () = (unsafe { foo_0() });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
