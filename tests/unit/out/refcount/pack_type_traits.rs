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
    assert!(conjunction_v_0.with(|rc| *rc.borrow()));
    assert!(conjunction_v_1.with(|rc| *rc.borrow()));
    assert!(!(conjunction_v_2.with(|rc| *rc.borrow())));
    assert!((conjunction_v_3.with(|rc| *rc.borrow())));
    assert!((!(conjunction_v_4.with(|rc| *rc.borrow()))));
    assert!(!(disjunction_v_5.with(|rc| *rc.borrow())));
    assert!(disjunction_v_6.with(|rc| *rc.borrow()));
    assert!(!(disjunction_v_7.with(|rc| *rc.borrow())));
    assert!((disjunction_v_8.with(|rc| *rc.borrow())));
    assert!((!(disjunction_v_9.with(|rc| *rc.borrow()))));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
