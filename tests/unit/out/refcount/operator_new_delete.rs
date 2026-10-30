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
    let mut a: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    a.write(42_());
    unimplemented!();
    unimplemented!();
    let mut arr: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    elem!(arr, 0_()).write(0_());
    elem!(arr, 1_()).write(1_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
