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
    let allocs: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(3_());
    let mut vec_: () = unimplemented!();
    unimplemented!().write(42_());
    0
}
pub fn __cpp2rust_init_globals() {}
