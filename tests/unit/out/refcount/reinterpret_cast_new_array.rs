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
    let mut arr: Ptr<()> =
        Ptr::alloc_array((0..2_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    elem!(arr, 0_()).write(67305985_());
    elem!(arr, 1_()).write(134678021_());
    let mut bytes: Ptr<()> = arr.reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(bytes, 0_()).write(170_());
    unimplemented!();
    elem!(bytes, 5_()).write(187_());
    unimplemented!();
    arr.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
