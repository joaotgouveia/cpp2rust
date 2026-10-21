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
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_(), 5_()])));
    let mut p0: Ptr<()> = ((a.as_pointer() as Ptr<()>).offset(unimplemented!()));
    let mut p1: Ptr<()> = ((a.as_pointer() as Ptr<()>).offset(4_()));
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
