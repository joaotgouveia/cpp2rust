extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let re: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut cs: Ptr<()> = Ptr::<()>::from_string_literal(b"aab");
    unimplemented!();
}
pub fn test02_1() {
    let re: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    ({ test02_1() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
