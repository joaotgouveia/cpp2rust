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
    let mut original: () = 67305985_();
    let mut halves: *mut () = ((&mut original as *mut ()) as *mut ());
    unimplemented!();
    unimplemented!();
    (*halves.offset((0_()) as isize)) = 4112_();
    unimplemented!();
    let mut arr: [(); 2] = [513_(), 1027_()];
    let mut as_int: *mut () = (arr.as_mut_ptr() as *mut ());
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
