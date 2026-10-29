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
    let neg: Value<()> = Rc::new(RefCell::new(-1_()));
    let mut words: Ptr<()> = (neg.as_pointer()).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    let neg64: Value<()> = Rc::new(RefCell::new(-256_()));
    let mut quarters: Ptr<()> = (neg64.as_pointer()).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
