extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn a_0() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut i_1));
}
static mut i_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 1_() });
pub unsafe fn b_2() -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut i_3));
}
static mut i_3: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 2_() });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
