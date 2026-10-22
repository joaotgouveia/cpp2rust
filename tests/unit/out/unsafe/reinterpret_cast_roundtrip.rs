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
    let mut original: i32 = (67305985_() as i32);
    let mut halves: *mut i16 = ((&mut original as *mut i32) as *mut i16);
    unimplemented!();
    unimplemented!();
    (*halves.offset((0_()) as isize)) = 4112_i16;
    unimplemented!();
    let mut arr: [i16; 2] = [513_i16, 1027_i16];
    let mut as_int: *mut i32 = (arr.as_mut_ptr() as *mut i32);
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
