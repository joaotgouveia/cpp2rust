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
    let f: Value<()> = Rc::new(RefCell::new(1_f32));
    let mut bits: Ptr<u32> = (f.as_pointer()).reinterpret_cast::<u32>();
    unimplemented!();
    bits.write(1073741824_u32);
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
