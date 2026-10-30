extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Ptr<()> = unimplemented!();
    let b: Ptr<()> = unimplemented!();
    return ({ (a.read()) } == { (b.read()) });
}
pub fn __cpp2rust_init_globals() {}
