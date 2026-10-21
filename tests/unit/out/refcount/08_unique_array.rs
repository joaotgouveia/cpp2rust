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
    let mut g: () = ({ make_unique_0(2_()) });
    unimplemented!().write(11_());
    unimplemented!().write(12_());
    let mut g_ptr: Ptr<()> = unimplemented!();
    elem!(g_ptr, unimplemented!()).write(13_());
    elem!(g_ptr, 1_()).write(14_());
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
