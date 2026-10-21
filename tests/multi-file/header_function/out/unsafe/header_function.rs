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
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn unrelated1_0() -> () {
    return 1_();
}
pub unsafe fn unrelated2_1() -> () {
    return 2_();
}
pub unsafe fn unrelated3_2() -> () {
    return 3_();
}
pub unsafe fn helper_3(mut x: ()) -> () {
    &(unsafe { unrelated1_0() });
    &(unsafe { unrelated2_1() });
    &(unsafe { unrelated3_2() });
    return ((x) + (1_()));
}
pub unsafe fn __cpp2rust_init_globals() {}
