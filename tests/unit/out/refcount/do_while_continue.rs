extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn run_0() -> () {
    let mut i: () = unimplemented!();
    let mut runs: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (((i < 4_()) as ()) != 0) {
        __do_while = false;
        runs += 1_();
        i += 1_();
        if (((i == 4_()) as ()) != 0) {
            continue 'loop_;
        }
    }
    return runs;
}
pub fn nested_1() -> () {
    let mut oi: () = unimplemented!();
    let mut runs: () = unimplemented!();
    let mut __do_while = true;
    'loop_: while __do_while || (((oi < 2_()) as ()) != 0) {
        __do_while = false;
        oi += 1_();
        let mut ii: () = unimplemented!();
        let mut __do_while = true;
        'loop_: while __do_while || (((ii < 3_()) as ()) != 0) {
            __do_while = false;
            runs += 1_();
            ii += 1_();
            if (((ii == 3_()) as ()) != 0) {
                continue 'loop_;
            }
        }
    }
    return runs;
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
