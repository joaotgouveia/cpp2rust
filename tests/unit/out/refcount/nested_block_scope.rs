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
    let mut x: () = 1_();
    {
        let mut x: () = 2_();
        unimplemented!();
        {
            let mut x: () = 3_();
            unimplemented!();
        }
        unimplemented!();
    }
    unimplemented!();
    let mut sum: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (i < 3_()) {
        let mut y: () = i;
        {
            let mut y: () = 10_();
            sum += y;
        }
        sum += y;
        i.postfix_inc();
    }
    unimplemented!();
    if (x == 1_()) {
        let mut x: () = 5_();
        {
            let mut x: () = 6_();
            unimplemented!();
        }
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
