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
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        1125912791875585_(),
        2251829878849541_(),
    ])));
    let mut words: Ptr<()> = (arr.as_pointer() as Ptr<()>).reinterpret_cast::<()>();
    let mut i: () = 0_();
    'loop_: while (i < 8_()) {
        unimplemented!();
        i.postfix_inc();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
