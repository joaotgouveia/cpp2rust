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
    let mut val: () = 578437695752307201_();
    let mut dwords: *mut () = ((&mut val as *mut ()) as *mut ());
    unimplemented!();
    unimplemented!();
    let mut words: *mut () = (dwords as *mut ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*words.offset((1_()) as isize)) = 48042_();
    unimplemented!();
    unimplemented!();
    (*dwords.offset((1_()) as isize)) = 4293844428_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
