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
    let mut vals: [i16; 3] = [(-1_() as i16), 256_i16, (-32768_() as i16)];
    let mut uvals: *mut u16 = (vals.as_mut_ptr() as *mut u16);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    (*uvals.offset((0_()) as isize)) = 42_u16;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
