extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let l1: Value<std_locale> = Rc::new(RefCell::new(std_locale::new_1({
        Ptr::<i8>::from_string_literal(b"C")
    })));
    let l2: Value<std_locale> = Rc::new(RefCell::new(std_locale::new_2(
        { ({ std_locale::classic() }) },
        { ({ use_facet_1(l1.as_pointer()) }) },
    )));
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
