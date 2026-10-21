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
    let original: Value<()> = Rc::new(RefCell::new(67305985_()));
    let mut halves: Ptr<()> = (original.as_pointer()).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    elem!(halves, unimplemented!()).write(4112_());
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([513_(), 1027_()])));
    let mut as_int: Ptr<()> = (arr.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
