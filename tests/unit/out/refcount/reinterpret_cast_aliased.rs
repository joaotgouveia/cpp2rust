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
    let mut view1: Ptr<u32> = (val.as_pointer()).reinterpret_cast::<u32>();
    let mut view2: Ptr<u32> = (val.as_pointer()).reinterpret_cast::<u32>();
    elem!(view1, 0_()).write((3721182122_() as u32));
    unimplemented!();
    unimplemented!();
    elem!(view2, 1_()).write((4293844428_() as u32));
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
