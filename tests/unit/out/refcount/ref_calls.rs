extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn bar_0() -> () {
    return 1_();
}
pub fn foo_1(x: Ptr<()>) -> Ptr<()> {
    return (x).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let x: Value<()> = Rc::new(RefCell::new(5_()));
    let y: Value<()> = Rc::new(RefCell::new((({ foo_1(x.as_pointer()) }).read())));
    let z: Ptr<()> = ({ foo_1(x.as_pointer()) });
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
