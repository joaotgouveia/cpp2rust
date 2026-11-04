extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn find_0(
    first: Ptr<i32>,
    mut _a1: Ptr<i32>,
    _a2: Ptr<i32>,
    _a3: std_input_iterator_tag,
) -> Ptr<i32> {
    let first: Value<Ptr<i32>> = Rc::new(RefCell::new(first));
    let _a3: Value<std_input_iterator_tag> = Rc::new(RefCell::new(_a3));
    return (*first.borrow()).clone();
}
pub fn test01_1() {
    let a: Value<std_input_iterator_tag> =
        Rc::new(RefCell::new(<std_input_iterator_tag>::default()));
    let i: Value<i32> = Rc::new(RefCell::new(0_i32));
    ({
        let _first: Ptr<i32> = (i.as_pointer());
        let _arg1: Ptr<i32> = (i.as_pointer());
        let _arg2: Value<i32> = Rc::new(RefCell::new(1));
        find_0(_first, _arg1, _arg2.as_pointer(), (*a.borrow()).clone())
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
