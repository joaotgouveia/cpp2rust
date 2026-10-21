extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn a_0() -> () {
    return i_1.with(|rc| *rc.borrow());
}
thread_local!(
    static i_1: Value<()> = Rc::new(RefCell::new(1_()));
);
pub fn b_2() -> () {
    return i_3.with(|rc| *rc.borrow());
}
thread_local!(
    static i_3: Value<()> = Rc::new(RefCell::new(2_()));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
