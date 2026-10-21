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
    let mut arr: [(); 2] = [1125912791875585_(), 2251829878849541_()];
    let mut words: *mut () = (arr.as_mut_ptr() as *mut ());
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (8_())) {
        unimplemented!();
        i.postfix_inc();
    }
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
