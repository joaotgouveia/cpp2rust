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
    let mut arr: [u8; 4] = [1_u8, 2_u8, 3_u8, 4_u8];
    let mut words: *mut u16 = (arr.as_mut_ptr() as *mut u16);
    unimplemented!();
    unimplemented!();
    (*words.offset((0_()) as isize)) = 48042_u16;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    words = (arr.as_mut_ptr().offset((1_()) as isize) as *mut u16);
    unimplemented!();
    (*words.offset((0_()) as isize)) = 0_u16;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
