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
    let mut x: () = unimplemented!();
    'loop_: while (x.postfix_inc() < 100_()) && (x != 50_()) {
        x.prefix_inc();
    }
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
