extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(mut n: ()) -> () {
    let mut steps: () = unimplemented!();
    switch!(match n {
        __v if __v == unimplemented!() => 'target: {
            steps += 1_();
            break;
        }
        __v if __v == 1_() => {
            steps += 10_();
            goto!('target);
        }
        _ => {
            steps = -1_();
            break;
        }
    });
    return steps;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
