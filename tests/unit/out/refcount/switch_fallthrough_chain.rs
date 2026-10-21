extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn fallthrough_chain_0(mut x: ()) -> () {
    let mut r: () = unimplemented!();
    switch!(match x {
        __v if __v == 1_() => {
            r += 1_();
        }
        __v if __v == 2_() => {
            r += 2_();
        }
        __v if __v == 3_() => {
            r += 4_();
        }
        __v if __v == 4_() => {
            r += 8_();
            break;
        }
        _ => {
            r = -1_();
            break;
        }
    });
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
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
