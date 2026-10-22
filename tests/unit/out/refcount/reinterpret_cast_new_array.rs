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
    let mut arr: Ptr<u32> = Ptr::alloc_array((0..2_()).map(|_| 0_u32).collect::<Box<[u32]>>());
    elem!(arr, 0_()).write(67305985_u32);
    elem!(arr, 1_()).write(134678021_u32);
    let mut bytes: Ptr<u8> = arr.reinterpret_cast::<u8>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(bytes, 0_()).write(170_u8);
    unimplemented!();
    elem!(bytes, 5_()).write(187_u8);
    unimplemented!();
    arr.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
