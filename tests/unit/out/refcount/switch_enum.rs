extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Color = ();
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
pub fn switch_enum_0(mut c: Color) -> () {
    'switch: {
        match { (c as ()) } {
            __v if __v == (Color_kRed as ()) => {
                return 10_();
            }
            __v if __v == (Color_kGreen as ()) => {
                return 20_();
            }
            __v if __v == (Color_kBlue as ()) => {
                return 30_();
            }
            _ => {}
        }
    };
    return -1_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
