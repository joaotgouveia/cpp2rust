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
    let mut buf: *mut () = (unimplemented!() as *mut ());
    (*buf.offset((unimplemented!()) as isize)) = 1_();
    (*buf.offset((1_()) as isize)) = 2_();
    (*buf.offset((2_()) as isize)) = 3_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut local: () = 4_();
    unimplemented!();
    unimplemented!();
    let mut other: *mut () = (unimplemented!() as *mut ());
    (*other) = 5_();
    unimplemented!();
    unimplemented!();
    let mut bytes: *mut () = (buf as *mut ());
    (*bytes.offset((unimplemented!()) as isize)) = 170_();
    (*bytes.offset((4_()) as isize)) = 187_();
    unimplemented!();
    unimplemented!();
    let mut words: [(); 2] = [16909060_(), 84281096_()];
    let mut halves: *mut () = (words.as_mut_ptr() as *mut ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
