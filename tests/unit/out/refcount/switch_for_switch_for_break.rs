extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_switch_for_break_0(mut n: ()) -> () {
    let mut r: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (i < n) {
        'switch: {
            match { i } {
                __v if __v == 1_() => {
                    let mut j: () = unimplemented!();
                    'loop_: while (j < 10_()) {
                        if (j == 2_()) {
                            break;
                        }
                        r += 1_();
                        j.prefix_inc();
                    }
                    r += 100_();
                    break 'switch;
                }
                _ => {
                    r += 10_();
                    break 'switch;
                }
            }
        };
        i.prefix_inc();
    }
    return r;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
