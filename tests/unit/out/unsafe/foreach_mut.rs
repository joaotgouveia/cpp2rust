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
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sum: () = unimplemented!();
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (*unimplemented!());
        sum += x.prefix_inc();
    }
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let x: () = (*unimplemented!());
        sum += x;
    }
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let x: *mut () = &mut (*unimplemented!());
        (*x) += 10_();
    }
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let x: *const () = &(*unimplemented!());
        sum += (*x);
    }
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: *mut () = (*unimplemented!());
        (*p) += 5_();
    }
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let p: *mut () = (*unimplemented!());
        sum += (*p);
    }
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: *mut () = (*unimplemented!());
        (*p) += 5_();
    }
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: *const () = (*unimplemented!()).cast_const();
        sum += (*p);
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
