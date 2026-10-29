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
    let mut val: () = 0_();
    let mut bytes: *mut () = ((&mut val as *mut ()) as *mut ());
    (*bytes.offset((0_()) as isize)) = 239_();
    (*bytes.offset((1_()) as isize)) = 190_();
    (*bytes.offset((2_()) as isize)) = 173_();
    (*bytes.offset((3_()) as isize)) = 222_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
