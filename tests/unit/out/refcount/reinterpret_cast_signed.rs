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
    let neg: Value<i32> = Rc::new(RefCell::new((-1_() as i32)));
    let mut words: Ptr<u16> = (neg.as_pointer()).reinterpret_cast::<u16>();
    unimplemented!();
    unimplemented!();
    let neg64: Value<i64> = Rc::new(RefCell::new((-256_() as i64)));
    let mut quarters: Ptr<i16> = (neg64.as_pointer()).reinterpret_cast::<i16>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
