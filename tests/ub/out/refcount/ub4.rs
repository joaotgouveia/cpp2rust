extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn smaller_0(x1: Ptr<()>, x2: Ptr<()>) -> Ptr<()> {
    return if ({ (x1.read()) } < { (x2.read()) }) {
        (x1).clone()
    } else {
        (x2).clone()
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut out: Ptr<()> = Ptr::<()>::null();
    let x1: Value<()> = Rc::new(RefCell::new(1_()));
    if ((*x1.borrow()) != 0) {
        let x2: Value<()> = Rc::new(RefCell::new(-1_()));
        out = ({ smaller_0(x1.as_pointer(), x2.as_pointer()) });
    }
    return (out.read());
}
pub fn __cpp2rust_init_globals() {}
