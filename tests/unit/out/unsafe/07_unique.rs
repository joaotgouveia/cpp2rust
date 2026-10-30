extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fn_0(mut u: ()) -> () {
    (*unimplemented!()) = 10_();
    return unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut f: () = unimplemented!();
    (*unimplemented!()) = 9_();
    let mut f_ptr1: *mut () = unimplemented!();
    (*f_ptr1) = 10_();
    let mut f_ptr2: *mut () = (&mut (*unimplemented!()) as *mut ());
    (*f_ptr2) = 11_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
