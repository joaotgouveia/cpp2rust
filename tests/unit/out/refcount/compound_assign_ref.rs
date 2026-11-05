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
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(10));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    {
        ({ (*v.borrow()).front_53() }).with_mut(|__v| *__v = *__v + 5)
    };
    ({ cpp2rust_assert_fail_0(((({ (*v.borrow()).front_53() }).read()) == 15)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
