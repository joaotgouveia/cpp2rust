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
    let mut u: Option<Value<i32>> = Ptr::alloc(3).to_owned_opt();
    assert!(u.is_some());
    let mut e: Option<Value<i32>> = None;
    assert!(!(e.is_some()));
    if u.is_some() {
        u.as_pointer().write(4);
    }
    assert!(((u.as_pointer().read()) == 4));
    let mut a: Option<Value<Box<[i32]>>> = Some(Rc::new(RefCell::new(
        (0..2_usize).map(|_| 0_i32).collect::<Box<[_]>>(),
    )));
    assert!(a.is_some());
    let mut ea: Option<Value<Box<[i32]>>> = None;
    assert!(!(ea.is_some()));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
