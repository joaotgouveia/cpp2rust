extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn borrow_in_condition_and_in_body_0(mut x: ()) -> () {
    switch!(match x {
        __v if __v == unimplemented!() => {}
        _ => {
            return (x + 1_());
        }
    });
    panic!("ub: non-void function does not return a value")
}
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
