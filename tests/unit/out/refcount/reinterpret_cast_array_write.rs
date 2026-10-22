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
    let arr: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([1_u8, 2_u8, 3_u8, 4_u8])));
    let mut words: Ptr<u16> = (arr.as_pointer() as Ptr<u8>).reinterpret_cast::<u16>();
    unimplemented!();
    unimplemented!();
    elem!(words, 0_()).write(48042_u16);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    words = (arr.as_pointer() as Ptr<u8>)
        .offset((1_()) as isize)
        .reinterpret_cast::<u16>();
    unimplemented!();
    elem!(words, 0_()).write(0_u16);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
