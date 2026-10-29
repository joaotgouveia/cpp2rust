extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn bump_0(mut arg: ()) -> () {
    let mut value: *mut () = (arg as *mut ());
    (*value) += 1_();
    return (*value);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut value: () = 41_();
    let mut opaque: () = ((&mut value as *mut ()) as ());
    let mut typed: *mut () = (opaque as *mut ());
    unimplemented!();
    unimplemented!();
    (*typed) = 7_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
