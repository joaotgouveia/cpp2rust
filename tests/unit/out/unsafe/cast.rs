extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type E = ();
pub const E_A: E = 0;
pub const E_B: E = 1;
pub const E_C: E = 2;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut size: () = 1_();
    unimplemented!();
    let mut ul: () = 5_();
    let mut s1: () = ul;
    unimplemented!();
    ul = (7_() as ());
    unimplemented!();
    let mut i: () = 2_();
    let mut e: E = ((i) as E);
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
