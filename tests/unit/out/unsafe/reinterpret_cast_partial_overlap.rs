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
    let mut arr: [u32; 2] = [67305985_u32, 134678021_u32];
    let mut bytes: *mut u8 = (arr.as_mut_ptr() as *mut u8);
    unimplemented!();
    unimplemented!();
    (*bytes.offset((3_()) as isize)) = 255_u8;
    unimplemented!();
    (*bytes.offset((4_()) as isize)) = 170_u8;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
