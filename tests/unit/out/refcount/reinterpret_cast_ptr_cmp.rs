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
    let i1: Value<()> = Rc::new(RefCell::new(42_()));
    let mut ptr1: Ptr<()> = (i1.as_pointer());
    let mut ptr2: Ptr<()> = (i1.as_pointer()).reinterpret_cast::<()>();
    let mut vptr1: () = (ptr1).to_any();
    let mut vptr2: () = (ptr2).to_any();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
