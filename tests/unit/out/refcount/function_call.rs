extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn function_0(mut y: (), mut z: ()) -> () {
    let mut x: () = 5_();
    return ((x + y) + z);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut y: () = ({ function_0(10_(), 1_()) });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
