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
    let N: Value<()> = Rc::new(RefCell::new(5_()));
    let mut arr1: [(); 5] = [0_(), 0_(), 0_(), 0_(), 0_()];
    let mut arr2: [(); 5] = [1_(), 1_(), 1_(), 1_(), 1_()];
    let mut i: () = 0_();
    'loop_: while (i < (*N.borrow())) {
        arr1[(i) as usize] = { (i + arr2[(i) as usize]) };
        i.prefix_inc();
    }
    let mut fatorial: () = 1_();
    let mut i: () = 0_();
    'loop_: while (i < (*N.borrow())) {
        fatorial *= arr1[(i) as usize];
        i.prefix_inc();
    }
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
