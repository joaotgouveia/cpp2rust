extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fopen_0(mut path: *const (), mut mode: *const ()) -> () {
    &(path);
    &(mode);
    return unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut fp: () = (unsafe {
        fopen_0(
            (c"irrelevant-file".as_ptr().cast_mut()).cast_const(),
            (c"r".as_ptr().cast_mut()).cast_const(),
        )
    });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
