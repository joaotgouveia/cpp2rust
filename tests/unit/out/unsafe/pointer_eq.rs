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
    let mut x: () = 5_();
    let mut p1: *const () = (&mut x as *mut ()).cast_const();
    let mut p2: *const () = (&mut x as *mut ()).cast_const();
    unimplemented!();
    let mut y: () = 5_();
    let mut p3: *const () = (&mut y as *mut ()).cast_const();
    unimplemented!();
    let mut arr: [(); 3] = [1_(), 2_(), 3_()];
    let mut p: *mut () = arr.as_mut_ptr();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut val: () = 42_();
    let mut orig: *mut () = (&mut val as *mut ());
    let mut as_bytes: *mut () = (orig as *mut ());
    let mut back: *mut () = (as_bytes as *mut ());
    unimplemented!();
    let mut arr_bytes: *mut () = (arr.as_mut_ptr() as *mut ());
    let mut arr_back: *mut () = (arr_bytes as *mut ());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
