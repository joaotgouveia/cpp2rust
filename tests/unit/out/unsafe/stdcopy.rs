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
    let mut input: [(); 3] = [1_(), 2_(), 3_()];
    let mut output: [(); 3] = [unimplemented!(); 3];
    (unsafe {
        let ___first: *mut () = input.as_mut_ptr();
        let ___last: *mut () = input.as_mut_ptr().offset((3_()) as isize);
        copy_0(___first, ___last, output.as_mut_ptr())
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
