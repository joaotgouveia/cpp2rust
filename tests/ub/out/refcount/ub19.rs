extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(mut array: Ptr<()>) {
    array.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: Ptr<()> = Ptr::alloc(1_());
    ({ foo_0((x).clone()) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
