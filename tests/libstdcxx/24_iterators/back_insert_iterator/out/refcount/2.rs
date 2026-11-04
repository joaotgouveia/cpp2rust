extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test02_0() {
    let li: Value<std_list_int__std_allocator_int__> =
        Rc::new(RefCell::new(<std_list_int__std_allocator_int__>::default()));
    let it: Value<std_back_insert_iterator_std_list_int__std_allocator_int___> =
        Rc::new(RefCell::new(({ back_inserter_1(li.as_pointer()) })));
    (*it.borrow_mut()) = { (*it.borrow()).clone() };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    ({ test02_0() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
