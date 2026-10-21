extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn switch_char_0(mut c: ()) -> () {
    'switch: {
        match { c } {
            __v if __v == ('a' as ()) => {
                return 1_();
            }
            __v if __v == ('b' as ()) => {
                return 2_();
            }
            __v if __v == ('\n' as ()) => {
                return 3_();
            }
            __v if __v == ('\0' as ()) => {
                return 4_();
            }
            _ => {
                return 0_();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub type Color = ();
pub const Color_kRed: Color = 0;
pub const Color_kGreen: Color = 1;
pub const Color_kBlue: Color = 2;
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
