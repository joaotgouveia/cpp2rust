extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut N: () = 3_();
    let mut arr: Ptr<()> =
        Ptr::alloc_array((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>());
    unimplemented!();
    let mut sum: () = 0_();
    let mut i: () = 0_();
    'loop_: while (i < N) {
        sum += { (elem!(arr, i).read()) };
        i.prefix_inc();
    }
    arr.delete();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
