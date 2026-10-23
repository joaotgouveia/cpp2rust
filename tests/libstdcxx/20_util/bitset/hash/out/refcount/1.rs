extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let b0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let h0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ (*h0.borrow_mut())(b0.as_pointer()) });
    let b1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let h1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ (*h1.borrow_mut())(b1.as_pointer()) });
    let b2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let h2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ (*h2.borrow_mut())(b2.as_pointer()) });
    let b3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let h3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ (*h3.borrow_mut())(b3.as_pointer()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_0() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
