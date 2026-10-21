extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fopen_0(mut path: Ptr<()>, mut mode: Ptr<()>) -> *mut ::libc::FILE {
    &(path);
    &(mode);
    return std::ptr::null_mut();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut fp: *mut ::libc::FILE = ({
        fopen_0(
            Ptr::<()>::from_string_literal(b"irrelevant-file"),
            Ptr::<()>::from_string_literal(b"r"),
        )
    });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
