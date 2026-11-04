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
    let ofs: Value<std_basic_ofstream_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_ofstream_char__std_char_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"./ofstream_default_openmode.txt") },
            None,
        ),
    ));
    let app: Value<std_basic_ofstream_char__std_char_traits_char__> = Rc::new(RefCell::new(
        std_basic_ofstream_char__std_char_traits_char__::new_1(
            { Ptr::<i8>::from_string_literal(b"./ofstream_default_openmode.txt") },
            { Some(app_0.with(|rc| *rc.borrow())) },
        ),
    ));
    ({
        cpp2rust_assert_fail_1(
            ((out_2.with(|rc| *rc.borrow()) as i32) != (app_0.with(|rc| *rc.borrow()) as i32)),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
