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
    let x: Value<()> = Rc::new(RefCell::new(1_()));
    let r: Ptr<()> = x.as_pointer();
    let mut y: () = 10_();
    r.write({ y });
    y += 1_();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
