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
    let mut fd: () = unimplemented!();
    unimplemented!();
    let mut buf: [(); 4] = [unimplemented!(); 4];
    let mut n: () = unimplemented!();
    return if ((((n) == (-1_())) as ()) != 0) {
        0_()
    } else {
        1_()
    };
}
pub unsafe fn __cpp2rust_init_globals() {}
