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
    let val: Value<u64> = Rc::new(RefCell::new(578437695752307201_u64));
    let mut dwords: Ptr<u32> = (val.as_pointer()).reinterpret_cast::<u32>();
    unimplemented!();
    unimplemented!();
    let mut words: Ptr<u16> = dwords.reinterpret_cast::<u16>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    elem!(words, 1_()).write(48042_u16);
    unimplemented!();
    unimplemented!();
    elem!(dwords, 1_()).write((4293844428_() as u32));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
