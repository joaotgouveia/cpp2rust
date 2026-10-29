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
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([10_(), 20_(), 30_(), 40_()])));
    let mut same: Ptr<()> = (arr.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(same, 2_()).write(99_());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
