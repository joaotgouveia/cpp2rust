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
    let vals: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
        (-1_() as i16),
        256_i16,
        (-32768_() as i16),
    ])));
    let mut uvals: Ptr<u16> = (vals.as_pointer() as Ptr<i16>).reinterpret_cast::<u16>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(uvals, 0_()).write(42_u16);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
