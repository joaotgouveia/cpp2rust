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
    let mut buf: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    elem!(buf, unimplemented!()).write(1_());
    elem!(buf, 1_()).write(2_());
    elem!(buf, 2_()).write(3_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut local: () = 4_();
    unimplemented!();
    unimplemented!();
    let mut other: Ptr<()> = unimplemented!().reinterpret_cast::<()>();
    other.write(5_());
    unimplemented!();
    unimplemented!();
    let mut bytes: Ptr<()> = buf.reinterpret_cast::<()>();
    elem!(bytes, unimplemented!()).write(170_());
    elem!(bytes, 4_()).write(187_());
    unimplemented!();
    unimplemented!();
    let words: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([16909060_(), 84281096_()])));
    let mut halves: Ptr<()> = (words.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
