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
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(10_());
    let b: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(20_());
    unimplemented!();
    let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(3_());
    let y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(4_());
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
