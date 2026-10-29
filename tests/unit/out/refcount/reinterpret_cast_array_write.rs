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
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    let mut words: Ptr<()> = (arr.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    elem!(words, 0_()).write(48042_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    words = (arr.as_pointer() as Ptr<()>)
        .offset((1_()) as isize)
        .reinterpret_cast::<()>();
    unimplemented!();
    elem!(words, 0_()).write(0_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
