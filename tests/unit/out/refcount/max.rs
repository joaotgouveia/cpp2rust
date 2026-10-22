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
    let x1: Value<()> = Rc::new(RefCell::new(1_()));
    let x2: Value<()> = Rc::new(RefCell::new(2_()));
    let mut x3: () = 10_();
    let mut x4: () = 20_();
    let mut p1: Ptr<()> = (x1.as_pointer());
    let mut p2: Ptr<()> = (x2.as_pointer());
    let mut r1: () = (unimplemented!().read());
    let mut r2: () = (unimplemented!().read());
    let mut r3: () = (unimplemented!().read());
    let mut r4: () = (unimplemented!().read());
    let mut r5: () = (unimplemented!().read());
    unimplemented!();
    let values: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 7_(), 7_(), 3_()])));
    let mut max: Ptr<()> = unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
