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
    let mut joined: *const () = c"alpha\nbeta\ngamma\n".as_ptr();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut arr: [(); 7] = std::mem::transmute(*b"foobar\0");
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut split_pieces: *const () = c"abcdefghi".as_ptr();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
