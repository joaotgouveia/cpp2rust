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
    let mut arr1: Ptr<()> =
        Ptr::alloc_array((0..100_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    elem!(arr1, 100_()).write(1_());
    arr1.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
