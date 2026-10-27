extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let mut b0: () = unimplemented!();
    let h0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut b1: () = unimplemented!();
    let h1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut b2: () = unimplemented!();
    let h2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut b3: () = unimplemented!();
    let h3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
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
