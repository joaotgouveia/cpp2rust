extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let x: Value<Vec<RandomAccessIterator<i32>>> = Rc::new(RefCell::new(
        (0..(2_usize) as usize)
            .map(|_| RandomAccessIterator::default())
            .collect::<Vec<_>>(),
    ));
    let i2: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let i3: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    let ci1: Value<RandomAccessIterator<i32>> = Rc::new(RefCell::new(
        <RandomAccessIterator<i32>>::from(({ (*i2.borrow()).clone() })),
    ));
    (*i2.borrow_mut()) = (*i3.borrow()).clone();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
