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
    let mut p: Ptr<u32> = Ptr::alloc(67305985_u32);
    let mut bytes: Ptr<u8> = p.reinterpret_cast::<u8>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(bytes, 0_()).write(16_u8);
    unimplemented!();
    p.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
