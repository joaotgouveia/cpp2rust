extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn smaller_0(x1: *mut (), x2: *mut ()) -> *mut () {
    return if ((*x1) < (*x2)) { (x1) } else { (x2) };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut out: *mut () = std::ptr::null_mut();
    let mut x1: () = 1_();
    if (x1 != 0) {
        let mut x2: () = -1_();
        out = (unsafe { smaller_0(&mut x1, &mut x2) });
    }
    return (*out);
}
pub unsafe fn __cpp2rust_init_globals() {}
