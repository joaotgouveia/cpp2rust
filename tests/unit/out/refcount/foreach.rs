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
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut i: () = 0_();
    'loop_: while (i < 10_()) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut sum: () = 0_();
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (unimplemented!().read());
        sum += x;
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
