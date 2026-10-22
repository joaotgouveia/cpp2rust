extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn second_row_0() -> Ptr<()> {
    let rows: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    return unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p: Ptr<()> = ({ second_row_0() });
    return (elem!(p, 0_()).read());
}
pub fn __cpp2rust_init_globals() {}
