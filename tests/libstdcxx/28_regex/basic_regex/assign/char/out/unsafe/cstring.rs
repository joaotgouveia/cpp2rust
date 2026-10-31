extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut re: () = unimplemented!();
    let mut cs: *const libc::c_char = c"aab".as_ptr();
    unimplemented!();
}
pub unsafe fn test02_1() {
    let mut re: () = unimplemented!();
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    (unsafe { test02_1() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
