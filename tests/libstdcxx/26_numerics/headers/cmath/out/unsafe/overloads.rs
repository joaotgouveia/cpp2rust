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
    let mut i: () = -1_();
    let mut j: () = 9_();
    let mut ans: () = unimplemented!();
    ans = (unsafe { abs_0(i) });
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = unimplemented!();
    ans = ans;
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
