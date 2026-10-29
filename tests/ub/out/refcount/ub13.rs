extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn escape_0(mut p: Ptr<()>) {
    p.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p1: Ptr<()> = Ptr::alloc(1_());
    ({ escape_0((p1).clone()) });
    return (p1.read());
}
pub fn __cpp2rust_init_globals() {}
