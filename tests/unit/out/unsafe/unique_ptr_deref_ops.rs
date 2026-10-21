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
    let mut p: std_unique_ptr_int__std_default_delete_int__ = unimplemented!();
    (*unimplemented!()) += 5_();
    (*unimplemented!()) -= 3_();
    (*unimplemented!()) *= 2_();
    let mut q: std_unique_ptr_int__std_default_delete_int__ = unimplemented!();
    let mut sum: () = ((*unimplemented!()) + (*unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
