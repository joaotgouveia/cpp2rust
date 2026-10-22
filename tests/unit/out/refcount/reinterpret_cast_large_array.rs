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
    let mut arr: Ptr<u32> = Ptr::alloc_array((0..N).map(|_| 0_u32).collect::<Box<[u32]>>());
    let mut i: () = 0_();
    'loop_: while (i < N) {
        elem!(arr, i).write(0_u32);
        i.postfix_inc();
    }
    elem!(arr, (N - 1_())).write((3148519816_() as u32));
    let mut words: Ptr<u16> = arr.reinterpret_cast::<u16>();
    unimplemented!();
    unimplemented!();
    arr.delete();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
