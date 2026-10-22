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
    let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut sum: () = 0_();
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (unimplemented!().read());
        sum += x.prefix_inc();
    }
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (unimplemented!().read());
        sum += x;
    }
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let x: Ptr<()> = unimplemented!();
        {
            x.with_mut(|__v| *__v = *__v + 10_())
        };
    }
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let x: Ptr<()> = unimplemented!();
        sum += { (x.read()) };
    }
    let v2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: Ptr<()> = (unimplemented!().read());
        {
            p.with_mut(|__v| *__v = *__v + 5_())
        };
    }
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: Ptr<()> = (unimplemented!().read());
        sum += { (p.read()) };
    }
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: Ptr<()> = (unimplemented!().read());
        {
            p.with_mut(|__v| *__v = *__v + 5_())
        };
    }
    'loop_: for p in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut p: Ptr<()> = (unimplemented!().read());
        sum += { (p.read()) };
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
