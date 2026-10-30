extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn find_0(first: Ptr<()>, mut _a1: Ptr<()>, _a2: Ptr<()>, _a3: ()) -> Ptr<()> {
    let first: Value<Ptr<()>> = Rc::new(RefCell::new(first));
    let _a3: Value<()> = Rc::new(RefCell::new(_a3));
    return (*first.borrow()).clone();
}
pub fn test01_1() {
    let a: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({
        let _first: Ptr<()> = (i.as_pointer());
        let _arg1: Ptr<()> = (i.as_pointer());
        let _arg2: Value<()> = Rc::new(RefCell::new(1_()));
        find_0(_first, _arg1, _arg2.as_pointer(), unimplemented!())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test01_1() });
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
