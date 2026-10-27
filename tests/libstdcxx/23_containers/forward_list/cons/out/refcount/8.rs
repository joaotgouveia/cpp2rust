extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let ni: Value<()> = Rc::new(RefCell::new(10_()));
    let i: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
    ])));
    let fl: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let flm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
