extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_unordered_set_0() {
    let mut tmp2: Ptr<()> = Ptr::<()>::null();
    tmp2 = Ptr::alloc(unimplemented!());
    unimplemented!();
    tmp2.delete();
}
pub fn test_unordered_map_1() {
    let mut tmp: Ptr<()> = Ptr::<()>::null();
    let mut i: () = 0_();
    'loop_: while (i < 20_()) {
        tmp = Ptr::alloc(unimplemented!());
        let mut j: () = 0_();
        'loop_: while (j < 10000_()) {
            let __rhs = j;
            unimplemented!().write(__rhs);
            j.postfix_inc();
        }
        tmp.delete();
        i.postfix_inc();
    }
    tmp = Ptr::alloc(unimplemented!());
    let mut i: () = 0_();
    'loop_: while (i < 150000_()) {
        unimplemented!();
        i.postfix_inc();
    }
    unimplemented!();
    tmp.delete();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    ({ test_unordered_set_0() });
    ({ test_unordered_map_1() });
    0
}
pub fn __cpp2rust_init_globals() {}
