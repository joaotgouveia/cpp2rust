extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn bump_0(mut arg: ()) -> i32 {
    let mut value: *mut i32 = (arg as *mut i32);
    (*value) = (((*value) as ()) + 1_()) as i32;
    return (*value);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut value: i32 = (41_() as i32);
    let mut opaque: () = ((&mut value as *mut i32) as ());
    let mut typed: *mut i32 = (opaque as *mut i32);
    unimplemented!();
    unimplemented!();
    (*typed) = (7_() as i32);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
