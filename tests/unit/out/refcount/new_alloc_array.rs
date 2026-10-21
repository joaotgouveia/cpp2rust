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
    let mut array: Ptr<()> =
        Ptr::alloc_array((0..100_()).map(|_| unimplemented!()).collect::<Box<[()]>>());
    unimplemented!();
    elem!(array, 99_()).write(-1_());
    let mut p1: Ptr<()> = (array).clone();
    'loop_: while ((p1.read()) >= unimplemented!()) {
        p1.write(1_());
        p1.prefix_inc();
    }
    let mut out: () = unimplemented!();
    let mut p1: Ptr<()> = (array).clone();
    'loop_: while ((p1.read()) >= unimplemented!()) {
        out += { (p1.read()) };
        p1.prefix_inc();
    }
    let mut p2: Ptr<()> = (array).clone();
    p2.delete();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
