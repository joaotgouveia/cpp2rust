extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let va: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    &(unimplemented!());
    &(unimplemented!());
    let cva: Ptr<()> = va.as_pointer();
    &(unimplemented!());
    &(unimplemented!());
}
pub fn test02_1() {
    let va: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    &(unimplemented!());
    &(unimplemented!());
    let cva: Ptr<()> = va.as_pointer();
    &(unimplemented!());
    &(unimplemented!());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    ({ test02_1() });
    0
}
pub fn __cpp2rust_init_globals() {}
