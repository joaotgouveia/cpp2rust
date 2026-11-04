extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let s: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"^$") },
            None,
        ),
    ));
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
