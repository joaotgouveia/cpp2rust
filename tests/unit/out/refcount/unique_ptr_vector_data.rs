extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0(u: Ptr<()>) -> Ptr<()> {
    return unimplemented!();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut p: Ptr<()> = unimplemented!();
    elem!(p, 2_()).write(1_());
    elem!(({ first_0(v.as_pointer(),) }), 0_()).write(5_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
