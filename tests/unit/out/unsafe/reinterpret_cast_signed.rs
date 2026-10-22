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
    let mut neg: i32 = (-1_() as i32);
    let mut words: *mut u16 = ((&mut neg as *mut i32) as *mut u16);
    unimplemented!();
    unimplemented!();
    let mut neg64: i64 = (-256_() as i64);
    let mut quarters: *mut i16 = ((&mut neg64 as *mut i64) as *mut i16);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
