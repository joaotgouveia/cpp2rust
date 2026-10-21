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
    let mut out: () = unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1_(),
        2_(),
        3_(),
        4_(),
        unimplemented!(),
    ])));
    let mut ptr: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(unimplemented!()));
    'loop_: while ((ptr.read()) != unimplemented!()) {
        out += { (ptr.read()) };
        ptr.prefix_inc();
    }
    let mut ptr: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(1_()));
    'loop_: while ((ptr.read()) != 4_()) {
        out += { (ptr.read()) };
        ptr.postfix_inc();
    }
    let mut ptr: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(4_()));
    'loop_: while ((ptr.read()) != 1_()) {
        out += { (ptr.read()) };
        ptr.postfix_dec();
    }
    let mut ptr: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(3_()));
    'loop_: while ((ptr.read()) != 2_()) {
        out += { (ptr.read()) };
        ptr.prefix_dec();
    }
    let mut ptr: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(unimplemented!()));
    'loop_: while ((ptr.read()) != unimplemented!()) {
        out += { (ptr.read()) };
        ptr = { ptr.offset((1_()) as isize) };
    }
    let mut ptr: Ptr<()> = ((arr.as_pointer() as Ptr<()>).offset(unimplemented!()));
    let mut i: () = unimplemented!();
    'loop_: while (i < 5_()) {
        out += { (elem!(ptr, i).read()) };
        i.prefix_inc();
    }
    unimplemented!();
    let mut step: () = 2_();
    let mut offset: Ptr<()> = (arr.as_pointer() as Ptr<()>).offset((step) as isize);
    unimplemented!();
    offset = { offset.offset(-((step) as isize)) };
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
