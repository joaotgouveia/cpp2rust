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
    let a: Value<()> = Rc::new(RefCell::new(1_()));
    let r: Ptr<()> = a.as_pointer();
    let p: Value<Ptr<()>> = Rc::new(RefCell::new((a.as_pointer())));
    r.write(2_());
    (*p.borrow()).write(3_());
    let mut p2: Ptr<Ptr<()>> = (p.as_pointer());
    (p2.read()).write(3_());
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
