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
    let vals: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([-1_(), 256_(), -32768_()])));
    let mut uvals: Ptr<()> = (vals.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(uvals, 0_()).write(42_());
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
