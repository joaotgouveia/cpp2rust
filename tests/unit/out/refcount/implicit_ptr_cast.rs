extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn write_ulong_0(mut p: Ptr<()>) {
    p.write(42_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ write_ulong_0((x.as_pointer())) });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
