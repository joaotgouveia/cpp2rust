extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test01_0() {
    let us1: Value<std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__> =
        Rc::new(RefCell::new(
            <std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__>::default(
            ),
        ));
    let us2: Value<std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__> =
        Rc::new(RefCell::new(
            <std_unordered_set_int__std_hash_int___std_equal_to_int___std_allocator_int__>::default(
            ),
        ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*us1.borrow()).insert_36(___x.as_pointer())
    });
    (*us2.borrow_mut()) = (*us1.borrow()).clone();
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*us1.borrow()).insert_36(___x.as_pointer())
    });
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
