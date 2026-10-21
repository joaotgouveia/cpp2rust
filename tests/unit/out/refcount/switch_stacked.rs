extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn stacked_0(mut x: ()) -> () {
    let mut r: () = 0_();
    'switch: {
        match { x } {
            __v if __v == 1_() || __v == 2_() || __v == 3_() => {
                r = 100_();
                break 'switch;
            }
            __v if __v == 4_() || __v == 5_() => {
                r = 200_();
                break 'switch;
            }
            _ => {
                r = 300_();
                break 'switch;
            }
        }
    };
    return r;
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
