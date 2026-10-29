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
    let val: Value<()> = Rc::new(RefCell::new(578437695752307201_()));
    let mut view1: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
    let mut view2: Ptr<()> = (val.as_pointer()).reinterpret_cast::<()>();
    elem!(view1, 0_()).write(3721182122_());
    unimplemented!();
    unimplemented!();
    elem!(view2, 1_()).write(4293844428_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
