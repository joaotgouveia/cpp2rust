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
    let mut src: [(); 3] = [1_(), 2_(), 3_()];
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let src1: [(); 3] = [1_(), 2_(), 3_()];
    let mut v4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut buf: [(); 5] = [10_(), 20_(), 30_(), 40_(), 50_()];
    let mut start: *const () = (buf.as_mut_ptr()).cast_const();
    let mut len: () = 5_();
    let mut v5: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut m: () = unimplemented!();
    (*unimplemented!()) = 10_();
    (*unimplemented!()) = 20_();
    (*unimplemented!()) = 30_();
    let mut v6: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
