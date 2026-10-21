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
    let x: Value<()> = Rc::new(RefCell::new(5_()));
    let mut p1: Ptr<()> = (x.as_pointer());
    let mut p2: Ptr<()> = (x.as_pointer());
    unimplemented!();
    let y: Value<()> = Rc::new(RefCell::new(5_()));
    let mut p3: Ptr<()> = (y.as_pointer());
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    let mut p: Ptr<()> = (arr.as_pointer() as Ptr<()>);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let val: Value<()> = Rc::new(RefCell::new(42_()));
    let mut orig: Ptr<()> = (val.as_pointer());
    let mut as_bytes: Ptr<()> = orig.reinterpret_cast::<()>();
    let mut back: Ptr<()> = as_bytes.reinterpret_cast::<()>();
    unimplemented!();
    let mut arr_bytes: Ptr<()> = (arr.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    let mut arr_back: Ptr<()> = arr_bytes.reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
