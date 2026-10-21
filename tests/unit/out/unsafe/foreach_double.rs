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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut square: () = unimplemented!();
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut e1: () = (*unimplemented!());
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut e2: () = (*unimplemented!());
            square += ((e1) * (e2));
        }
    }
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let e1: *mut () = &mut (*unimplemented!());
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let e2: *const () = &(*unimplemented!());
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let e1: *const () = &(*unimplemented!());
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let e2: *mut () = &mut (*unimplemented!());
            square += ((*e1) * (*e2));
        }
    }
    'loop_: for e1 in IterRange::new(unimplemented!(), unimplemented!()) {
        let e1: *mut () = &mut (*unimplemented!());
        'loop_: for e2 in IterRange::new(unimplemented!(), unimplemented!()) {
            let e2: *mut () = &mut (*unimplemented!());
            square += ((*e1) * (*e2));
        }
    }
    let mut m: () = unimplemented!();
    let mut v1: () = unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    'loop_: for row in IterRange::new(unimplemented!(), unimplemented!()) {
        let row: *mut () = &mut (*unimplemented!());
        'loop_: for col in IterRange::new(unimplemented!(), unimplemented!()) {
            let col: *mut () = &mut (*unimplemented!());
            square += (*col);
        }
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
