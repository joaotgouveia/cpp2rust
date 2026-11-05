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
    let i: Value<i32> = Rc::new(RefCell::new(0));
    'loop_: while ((*i.borrow()) < 10) {
        ({ (*v.borrow()).push_back_59(i.as_pointer()) });
        (*i.borrow_mut()).prefix_inc();
    }
    let mut sum: i32 = 0;
    'loop_: for x in IterRange::new(({ (*v.borrow()).begin_28() }), ({ (*v.borrow()).end_30() })) {
        let mut x: i32 = (x.read());
        sum += x;
    }
    ({ cpp2rust_assert_fail_0((sum == 45)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
