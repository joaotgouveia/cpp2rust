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
    let mut x1: () = 1_();
    let mut x2: () = 2_();
    let mut x3: () = 10_();
    let mut x4: () = 20_();
    let mut p1: *mut () = (&mut x1 as *mut ());
    let mut p2: *mut () = (&mut x2 as *mut ());
    let mut r1: () = (*unimplemented!());
    let mut r2: () = (*unimplemented!());
    let mut r3: () = (*unimplemented!());
    let mut r4: () = (*unimplemented!());
    let mut r5: () = (*unimplemented!());
    unimplemented!();
    let mut values: [(); 4] = [1_(), 7_(), 7_(), 3_()];
    let mut max: *mut () = unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
