extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let v: Value<Vec<i32>> = Rc::new(RefCell::new(Vec::new()));
    let vi: Value<RandomAccessIterator<i32>> =
        Rc::new(RefCell::new(RandomAccessIterator::default()));
    {
        let __a1 = 1;
        (*v.borrow_mut()).push(__a1)
    };
    (*vi.borrow_mut()) = v.decay().end();
    (*vi.borrow()) != v.decay().begin();
    (*vi.borrow()) > v.decay().begin();
    (*vi.borrow()) <= v.decay().begin();
    (*vi.borrow()) >= v.decay().begin();
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
