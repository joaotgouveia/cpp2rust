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
    let mut arr: [u8; 4] = [10_u8, 20_u8, 30_u8, 40_u8];
    let mut same: *mut u8 = arr.as_mut_ptr();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*same.offset((2_()) as isize)) = 99_u8;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
