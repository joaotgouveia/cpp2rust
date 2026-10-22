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
    let value: Value<u32> = Rc::new(RefCell::new(67305985_u32));
    let mut bytes: Ptr<u8> = (value.as_pointer()).reinterpret_cast::<u8>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8])));
    let mut arr16: Ptr<u16> = (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u16>();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
