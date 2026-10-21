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
    let mut a: () = 1_();
    let mut p: *mut () = (&mut a as *mut ());
    (unsafe { libc::printf(c"%p %d".as_ptr() as *const libc::c_char, (p), (*p)) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
