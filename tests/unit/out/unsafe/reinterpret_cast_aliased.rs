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
    let mut val: u64 = 578437695752307201_u64;
    let mut view1: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    let mut view2: *mut u32 = ((&mut val as *mut u64) as *mut u32);
    (*view1.offset((0_()) as isize)) = (3721182122_() as u32);
    unimplemented!();
    unimplemented!();
    (*view2.offset((1_()) as isize)) = (4293844428_() as u32);
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
