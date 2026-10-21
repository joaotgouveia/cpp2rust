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
    let mut v: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (10_())) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut sum: () = unimplemented!();
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (*unimplemented!());
        sum += x;
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
