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
    let tmp: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    let mut j: i32 = 0;
    'loop_: while (j < 2000) {
        let i: Value<i32> = Rc::new(RefCell::new(0));
        'loop_: while ((*i.borrow()) < 10000) {
            ({ (*tmp.borrow()).push_back_59(i.as_pointer()) });
            (*i.borrow_mut()).postfix_inc();
        }
        j.postfix_inc();
    }
    0
}
pub fn __cpp2rust_init_globals() {}
