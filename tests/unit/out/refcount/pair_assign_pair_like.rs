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
    let a: Value<(Value<i32>, Value<i64>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new(1)),
        Rc::new(RefCell::new(2_i64)),
    )));
    let b: Value<(Value<i16>, Value<i32>)> = Rc::new(RefCell::new((
        Rc::new(RefCell::new((7 as i16))),
        Rc::new(RefCell::new(8)),
    )));
    a.as_pointer().write((
        Rc::new(RefCell::new((((*b.borrow()).0.as_pointer().read()) as i32))),
        Rc::new(RefCell::new((((*b.borrow()).1.as_pointer().read()) as i64))),
    ));
    assert!(({ ((*(*a.borrow()).0.borrow()) == 7) && ((*(*a.borrow()).1.borrow()) == 8_i64) }));
    (*(*b.borrow()).0.borrow_mut()) = 9_i16;
    assert!(({ ((*(*a.borrow()).0.borrow()) == 7) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
