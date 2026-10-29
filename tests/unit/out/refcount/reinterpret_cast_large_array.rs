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
    let mut N: () = 10000_();
    let mut arr: Ptr<()> =
        Ptr::alloc_array((0..N).map(|_| unimplemented!()).collect::<Box<[()]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        elem!(arr, i).write(0_());
        i.postfix_inc();
    }
    elem!(arr, (N - 1_())).write(3148519816_());
    let mut words: Ptr<()> = arr.reinterpret_cast::<()>();
    unimplemented!();
    unimplemented!();
    arr.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
