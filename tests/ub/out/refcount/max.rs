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
    let a: Ptr<()> = ({
        let ___a: Value<()> = Rc::new(RefCell::new(1_()));
        let ___b: Value<()> = Rc::new(RefCell::new(2_()));
        min_0(___a.as_pointer(), ___b.as_pointer())
    });
    let b: Ptr<()> = ({
        let ___a: Value<()> = Rc::new(RefCell::new(1_()));
        let ___b: Value<()> = Rc::new(RefCell::new(2_()));
        max_1(___a.as_pointer(), ___b.as_pointer())
    });
    return ({ (a.read()) } == { (b.read()) });
}
pub fn __cpp2rust_init_globals() {}
