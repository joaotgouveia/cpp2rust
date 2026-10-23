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
    let mut x: () = unimplemented!();
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let c: *mut () = &mut (*unimplemented!());
        (*c).prefix_inc();
    }
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let c: *mut () = &mut (*unimplemented!());
        (unsafe { libc::printf(c"%c\n".as_ptr() as *const libc::c_char, (*c)) });
    }
    'loop_: for c in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut c: () = (*unimplemented!());
        (unsafe { libc::printf(c"%c\n".as_ptr() as *const libc::c_char, (c)) });
    }
    let mut v: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: *mut () = (*unimplemented!());
        (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, (*p)) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
