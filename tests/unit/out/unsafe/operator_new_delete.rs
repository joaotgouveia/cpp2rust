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
    let mut a: *mut () = (unimplemented!() as *mut ());
    (*a) = 42_();
    unimplemented!();
    unimplemented!();
    let mut arr: *mut () = (unimplemented!() as *mut ());
    (*arr.offset((0_()) as isize)) = 0_();
    (*arr.offset((1_()) as isize)) = 1_();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
