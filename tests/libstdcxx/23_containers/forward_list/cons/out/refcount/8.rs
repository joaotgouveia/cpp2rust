extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let ni: Value<i32> = Rc::new(RefCell::new(10));
    let i: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9])));
    let fl: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let flm: Value<()> = Rc::new(RefCell::new(unimplemented!()));
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
