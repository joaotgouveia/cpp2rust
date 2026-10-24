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
    let mut g: () = (unsafe { make_unique_0(2_()) });
    (*unimplemented!()) = 11_();
    (*unimplemented!()) = 12_();
    let mut g_ptr: *mut () = unimplemented!();
    (*g_ptr.offset((0_()) as isize)) = 13_();
    (*g_ptr.offset((1_()) as isize)) = 14_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
