extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let mut b0: Vec<bool> = unimplemented!();
    let h0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut b1: Vec<bool> = vec![false; 10_usize];
    let h1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut b2: Vec<bool> = vec![false; 100_usize];
    let h2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    let mut b3: Vec<bool> = vec![false; 1000_usize];
    let h3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
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
