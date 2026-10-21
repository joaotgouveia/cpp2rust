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
    let rows: Value<Box<[Box<[()]>]>> = Rc::new(RefCell::new(Box::new([
        Box::new([1_(), 2_(), 3_()]),
        Box::new([4_(), 5_(), 6_()]),
    ])));
    return ((((rows.as_pointer() as Ptr<Box<[()]>>)
        .offset(1_())
        .as_pointer()) as Ptr<()>)
        - (((((rows.as_pointer() as Ptr<Box<[()]>>)
            .offset(unimplemented!())
            .as_pointer()) as Ptr<()>)
            .offset(unimplemented!()))
        .offset((2_()) as isize))) as ();
}
pub fn __cpp2rust_init_globals() {}
