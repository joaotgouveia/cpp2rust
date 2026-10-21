extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn mixed_return_break_0(mut x: ()) -> () {
    let mut r: () = -1_();
    'switch: {
        match { x } {
            __v if __v == unimplemented!() => {
                return 100_();
            }
            __v if __v == 1_() => {
                r = 10_();
                break 'switch;
            }
            __v if __v == 2_() => {
                return 200_();
            }
            _ => {
                r = 99_();
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
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
