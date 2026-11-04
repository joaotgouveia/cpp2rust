extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let v: Value<std_vector_bool__std_allocator_bool__> = Rc::new(RefCell::new(
        <std_vector_bool__std_allocator_bool__>::default(),
    ));
    ({
        let ___first: std__Bit_iterator = ({ (*v.borrow()).begin_23() });
        let ___last: std__Bit_iterator = ({ (*v.borrow()).begin_23() });
        let ___value: Value<bool> = Rc::new(RefCell::new(false));
        fill_1(___first, ___last, ___value.as_pointer())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    0
}
pub fn __cpp2rust_init_globals() {}
