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
    let mut x1: () = 1_();
    let mut x2: () = 2_();
    let mut x3: () = 4_();
    let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    unimplemented!();
    let mut sum: () = 0_();
    'loop_: for elem in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut elem: () = (unimplemented!().read());
        sum += elem;
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
