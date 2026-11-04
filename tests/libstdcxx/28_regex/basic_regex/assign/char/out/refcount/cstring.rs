extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        <std_basic_regex_char__std_regex_traits_char__>::default(),
    ));
    let mut cs: Ptr<i8> = Ptr::<i8>::from_string_literal(b"aab");
    ({ (*re.borrow()).assign_17((cs).clone(), None) });
}
pub fn test02_1() {
    let re: Value<std_basic_regex_char__std_regex_traits_char__> = Rc::new(RefCell::new(
        std_basic_regex_char__std_regex_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"[[:alnum:]]") },
            { basic_2.with(|rc| *rc.borrow()) },
        ),
    ));
    (*re.borrow_mut()) = Ptr::<i8>::from_string_literal(b"\\w+");
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test01_0() });
    ({ test02_1() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
