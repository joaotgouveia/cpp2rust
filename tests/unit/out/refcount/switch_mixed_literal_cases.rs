extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn mixed_literal_cases_0(mut x: ()) -> () {
    'switch: {
        match { x } {
            __v if __v == -1_() => {
                return 1_();
            }
            __v if __v == 16_() => {
                return 2_();
            }
            __v if __v == 65152_() => {
                return 3_();
            }
            __v if __v == -255_() => {
                return 4_();
            }
            _ => {
                return unimplemented!();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
