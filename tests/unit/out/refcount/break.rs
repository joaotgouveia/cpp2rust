extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn for_test_0(mut n: ()) -> () {
    let mut x: () = unimplemented!();
    let mut i: () = unimplemented!();
    let mut j: () = unimplemented!();
    'loop_: while (i < n) {
        x += 1_();
        if (x == 100_()) {
            break;
        }
        let mut k: () = unimplemented!();
        let mut w: () = unimplemented!();
        'loop_: while (w < j) {
            break;
            {
                {
                    w += 1_();
                    k += 1_()
                };
                i += k
            };
        }
        x = { (x + 1_()) };
        {
            j = i;
            i += 1_()
        };
    }
    return x;
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
