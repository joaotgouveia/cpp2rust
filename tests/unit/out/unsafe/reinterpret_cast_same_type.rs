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
    let mut arr: [(); 4] = [10_(), 20_(), 30_(), 40_()];
    let mut same: *mut () = arr.as_mut_ptr();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*same.offset((2_()) as isize)) = 99_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
