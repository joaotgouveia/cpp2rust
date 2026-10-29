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
    let mut v: std_vector_bool__std_allocator_bool__ =
        std_vector_bool__std_allocator_bool__::new_1({ unimplemented!() }, None);
    unimplemented!();
    let mut r: std__Bit_reference = v[(0_()) as usize];
    let mut copy: std__Bit_reference = r;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
