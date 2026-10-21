extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_on_assignment_0(mut x: ()) -> () {
    let mut y: () = unimplemented!();
    let mut r: () = unimplemented!();
    'switch: {
        match {
            {
                y = (x + 1_());
                y
            }
        } {
            __v if __v == 1_() => {
                r = 10_();
                break 'switch;
            }
            __v if __v == 2_() => {
                r = 20_();
                break 'switch;
            }
            _ => {
                r = y;
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
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
