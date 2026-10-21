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
    let mut x: () = {
        let mut a: () = 1_();
        let mut b: () = 2_();
        (a + b)
    };
    unimplemented!();
    let mut counter: () = unimplemented!();
    let mut y: () = {
        counter.postfix_inc();
        (counter * 10_())
    };
    unimplemented!();
    unimplemented!();
    let mut z: () = {
        let mut v: () = 5_();
        if (v > unimplemented!()) {
            v = { (v * 2_()) };
        }
        v
    };
    unimplemented!();
    unimplemented!();
    {
        counter.postfix_inc()
    };
    {
        if (counter != 2_()) {
            counter = 2_();
        }
    };
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
