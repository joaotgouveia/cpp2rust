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
    let src: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut v1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut v3: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let src1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut v4: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let buf: Value<Box<[()]>> =
        Rc::new(RefCell::new(Box::new([10_(), 20_(), 30_(), 40_(), 50_()])));
    let mut start: Ptr<()> = (buf.as_pointer() as Ptr<()>);
    let mut len: () = 5_();
    let mut v5: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(10_());
    unimplemented!().write(20_());
    unimplemented!().write(30_());
    let v6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
