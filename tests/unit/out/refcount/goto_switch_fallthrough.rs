extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn sm_0(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = 0_();
            switch!(match n {
                __v if __v == 0_() => {
                    ret += 1_();
                }
                __v if __v == 1_() => {
                    ret += 10_();
                    goto!('out);
                }
                _ => {
                    ret += 100_();
                    break;
                }
            });
            ret += 1000_();
        }
        'out: {
            return ret;
        }
    });
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
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
