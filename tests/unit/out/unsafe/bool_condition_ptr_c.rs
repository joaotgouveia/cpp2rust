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
    let mut storage: () = 7_();
    let mut p: *mut () = (&mut storage as *mut ());
    let mut np: *mut () = std::ptr::null_mut();
    if !(p).is_null() {
        unimplemented!();
    }
    if !(!(p).is_null()) {
        unimplemented!();
    }
    if !(np).is_null() {
        unimplemented!();
    }
    if !(!(np).is_null()) {
        unimplemented!();
    }
    let mut iter: *mut () = p;
    let mut iters: () = unimplemented!();
    'loop_: while !(iter).is_null() {
        iters.prefix_inc();
        iter = std::ptr::null_mut();
    }
    unimplemented!();
    let mut t3: () = if !(p).is_null() {
        1_()
    } else {
        unimplemented!()
    };
    unimplemented!();
    let mut t4: () = if !(np).is_null() {
        1_()
    } else {
        unimplemented!()
    };
    unimplemented!();
    let mut t5: () = (!(!(p).is_null()) as ());
    unimplemented!();
    let mut t6: () = (!(!(np).is_null()) as ());
    unimplemented!();
    let mut b2: () = !(p).is_null();
    let mut b3: () = !(np).is_null();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
