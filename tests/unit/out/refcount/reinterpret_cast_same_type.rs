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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([10_u8, 20_u8, 30_u8, 40_u8])));
    let mut same: Ptr<u8> = (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u8>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(same, 2_()).write(99_u8);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
