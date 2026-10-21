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
    let mut p: Ptr<()> = Ptr::alloc(67305985_());
    let mut bytes: Ptr<()> = p.reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(bytes, unimplemented!()).write(16_());
    unimplemented!();
    p.delete();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
