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
    let liit: Value<BidirectionalIterator<i32>> =
        Rc::new(RefCell::new(({ (*li.borrow()).begin_24() })));
    let it01: Value<std_insert_iterator_std_list_int__std_allocator_int___> = Rc::new(
        RefCell::new(std_insert_iterator_std_list_int__std_allocator_int___::new(
            { li.as_pointer() },
            { (*liit.borrow()).clone() },
        )),
    );
    let it02: Value<std_insert_iterator_std_list_int__std_allocator_int___> = Rc::new(
        RefCell::new(({ inserter_1(li.as_pointer(), (*liit.borrow()).clone()) })),
    );
    (*it01.borrow_mut()) = { (*it01.borrow()).clone() };
    (*it02.borrow_mut()) = { (*it02.borrow()).clone() };
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
