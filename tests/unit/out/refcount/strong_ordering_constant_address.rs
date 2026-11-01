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
fn main_0() -> i32 {
    let e: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_equal();
    let g: Ptr<std::cmp::Ordering> = libcc2rs::strong_ordering_greater();
    assert!(({ (e).clone() } == { (libcc2rs::strong_ordering_equal()) }));
    assert!(({ (g).clone() } == { (libcc2rs::strong_ordering_greater()) }));
    assert!(({ (e).clone() } != { (g).clone() }));
    assert!(((libcc2rs::strong_ordering_less()) != (libcc2rs::strong_ordering_equivalent())));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
