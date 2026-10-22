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
    let d: Value<()> = Rc::new(RefCell::new(1_f64));
    let mut bits: Ptr<u64> = (d.as_pointer()).reinterpret_cast::<u64>();
    unimplemented!();
    bits.write(4614256656552045848_u64);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
