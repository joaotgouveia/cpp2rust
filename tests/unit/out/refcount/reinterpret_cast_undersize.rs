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
    let b: Value<()> = Rc::new(RefCell::new(66_()));
    let mut p: Ptr<()> = (b.as_pointer()).reinterpret_cast::<()>();
    let mut val: () = (p.read());
    &(val);
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
