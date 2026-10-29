extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn escape_0(mut ptr: Ptr<()>) {
    ptr.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut alloc: Ptr<()> = Ptr::alloc(1_());
    ({ escape_0((alloc).clone()) });
    alloc.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
