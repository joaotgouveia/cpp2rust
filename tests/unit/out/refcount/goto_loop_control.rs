extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn loopctl_0() -> () {
    let mut sum: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while (((i < 5_()) as ()) != 0) {
        goto_block!({
            '__entry: {
                if (((i == 1_()) as ()) != 0) {
                    i.postfix_inc();
                    continue 'loop_;
                }
                if (((i == 4_()) as ()) != 0) {
                    break;
                }
                goto!('add);
            }
            'add: {
                sum += 1_();
            }
        });
        i.postfix_inc();
    }
    return sum;
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
