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
    let arr: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([67305985_u32, 134678021_u32])));
    let mut bytes: Ptr<u8> = (arr.as_pointer() as Ptr<u32>).reinterpret_cast::<u8>();
    unimplemented!();
    unimplemented!();
    elem!(bytes, 3_()).write(255_u8);
    unimplemented!();
    elem!(bytes, 4_()).write(170_u8);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
