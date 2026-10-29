extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type E = ();
pub const E_A: E = 0;
pub const E_B: E = 1;
pub const E_C: E = 2;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut size: () = 1_();
    unimplemented!();
    let mut ul: () = 5_();
    let mut s1: () = ul;
    unimplemented!();
    ul = (7_() as ());
    unimplemented!();
    let mut i: () = 2_();
    let mut e: E = ((i) as E);
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
