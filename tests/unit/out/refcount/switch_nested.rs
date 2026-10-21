extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn nested_0(mut a: (), mut b: ()) -> () {
    let mut r: () = unimplemented!();
    'switch: {
        match { a } {
            __v if __v == 1_() => {
                'switch: {
                    match { b } {
                        __v if __v == 10_() => {
                            r = 11_();
                            break 'switch;
                        }
                        __v if __v == 20_() => {
                            r = 12_();
                            break 'switch;
                        }
                        _ => {
                            r = 13_();
                            break 'switch;
                        }
                    }
                };
                r += 1_();
                break 'switch;
            }
            __v if __v == 2_() => {
                r = 2_();
                break 'switch;
            }
            _ => {
                r = -1_();
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
