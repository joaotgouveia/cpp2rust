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
    let mut arr2: [(); 2] = [2_(), 2_()];
    arr2[(0_()) as usize] = 3_();
    arr2[(1_()) as usize] = 4_();
    let mut arr2_ptr: *mut () = arr2.as_mut_ptr();
    (*arr2_ptr.offset((0_()) as isize)) = 5_();
    (*arr2_ptr.offset((1_()) as isize)) = 6_();
    let arr2_ref1: *mut () = &mut arr2[(1_()) as usize];
    (*arr2_ref1) = 7_();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
