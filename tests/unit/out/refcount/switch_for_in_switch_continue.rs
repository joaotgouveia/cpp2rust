extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_in_switch_continue_0(mut n: ()) -> () {
    let mut r: () = unimplemented!();
    'switch: {
        match { n } {
            __v if __v == unimplemented!() => {
                let mut i: () = unimplemented!();
                'loop_: while (i < 5_()) {
                    if ((i % 2_()) == unimplemented!()) {
                        i.prefix_inc();
                        continue 'loop_;
                    }
                    r += i;
                    i.prefix_inc();
                }
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
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
