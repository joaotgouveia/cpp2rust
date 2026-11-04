extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn All_0(
    arr: Ptr<std_unique_ptr_intarrarr__std_default_delete_intarrarr__>,
    mut N: i32,
    mut element: i32,
) {
    let all: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_1((N as u64)) })));
    let mut i: i32 = 0;
    'loop_: while (i < N) {
        (*all.borrow_mut())[(i as u64)] = element;
        i.prefix_inc();
    }
    arr.write({ (*all.borrow()).clone() });
}
pub fn Consume_2(arr: std_unique_ptr_intarrarr__std_default_delete_intarrarr__, mut N: i32) -> i32 {
    let arr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(arr));
    let sum: Value<i32> = Rc::new(RefCell::new(0));
    let mut i: i32 = -1_i32;
    'loop_: while ((*i.prefix_inc()) < N) {
        (*sum.borrow_mut()) += { (*arr.borrow())[(i as u64)] };
    }
    return (*sum.borrow());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut N: i32 = 10;
    let arr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_1((N as u64)) })));
    ({ All_0(arr.as_pointer(), N, 1) });
    ({ cpp2rust_assert_fail_3((({ Consume_2((*arr.borrow_mut()).clone(), N) }) == 10)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
