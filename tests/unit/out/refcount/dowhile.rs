extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn dowhile_0(mut x: ()) -> () {
    let mut __do_while = true;
    'loop_: while __do_while || (x <= 200_()) {
        __do_while = false;
        x += 1_();
        let mut __do_while = true;
        'loop_: while __do_while || (x <= 100_()) {
            __do_while = false;
            x += 1_();
            x += 1_();
        }
        x += 1_();
    }
    return x;
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
