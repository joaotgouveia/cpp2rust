extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_ALPHA: anon_0 = 7;
pub unsafe fn a_value_1() -> () {
    let mut x: () = unimplemented!();
    x |= (anon_0_ALPHA as i32);
    return x;
}
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
pub type anon_2 = ();
pub const anon_2_BETA: anon_2 = 9;
pub unsafe fn b_value_3() -> () {
    let mut x: () = unimplemented!();
    x |= (anon_2_BETA as i32);
    return x;
}
pub unsafe fn __cpp2rust_init_globals() {}
