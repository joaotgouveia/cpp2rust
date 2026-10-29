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
    let mut value: () = 67305985_();
    let mut bytes: *mut () = ((&mut value as *mut ()) as *mut ());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut arr: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let mut arr16: *mut () = (arr.as_mut_ptr() as *mut ());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
