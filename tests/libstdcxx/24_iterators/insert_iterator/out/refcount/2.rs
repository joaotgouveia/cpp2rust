extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test02_0() {
    let li: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let liit: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it01: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let it02: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test02_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
