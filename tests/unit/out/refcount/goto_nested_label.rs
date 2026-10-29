extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn scan_0(mut n: ()) -> () {
    let mut total: () = 0_();
    let mut i: () = 0_();
    'loop_: while (((i < n) as ()) != 0) {
        let mut j: () = unimplemented!();
        goto_block!({
            '__entry: {
                j = 0_();
                'loop_: while (((j < 10_()) as ()) != 0) {
                    if (((j == 5_()) as ()) != 0) {
                        goto!('next);
                    }
                    total += 1_();
                    j.postfix_inc();
                }
                total += 100_();
            }
            'next: {
                total += 1000_();
            }
        });
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
