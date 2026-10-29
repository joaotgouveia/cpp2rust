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
    let mut a: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
    let mut p0: *const () = (&mut a[(0_()) as usize] as *mut ()).cast_const();
    let mut p1: *const () = (&mut a[(4_()) as usize] as *mut ()).cast_const();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
