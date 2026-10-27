extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    {
        let list1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let list2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        let node_of_interest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let list1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let list2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut i: () = 0_();
        'loop_: while (i < 10_()) {
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        let node_of_interest: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
