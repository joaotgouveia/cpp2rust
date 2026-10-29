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
    let mut fname: *const () = c"testfile.txt".as_ptr();
    let mut mode: *const () = c"rb".as_ptr();
    let mut file_ptr: () = unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
