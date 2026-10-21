extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0(a: Ptr<()>) -> Ptr<()> {
    return (a).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<()> = Rc::new(RefCell::new(1_()));
    let mut pa: Ptr<()> = (a.as_pointer());
    let b: Ptr<()> = ({ foo_0((pa).clone()) });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
