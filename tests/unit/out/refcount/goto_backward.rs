extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn retry_0(mut n: ()) -> () {
    let mut count: () = unimplemented!();
    let mut acc: () = unimplemented!();
    goto_block!({
        '__entry: {
            count = unimplemented!();
            acc = unimplemented!();
        }
        'again: {
            count += 1_();
            acc += n;
            if (((count < 3_()) as ()) != 0) {
                goto!('again);
            }
            return acc;
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
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
