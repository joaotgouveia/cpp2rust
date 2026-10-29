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
    mut N: (),
    mut element: (),
) {
    let all: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_1(N) })));
    let mut i: () = 0_();
    'loop_: while (i < N) {
        (*all.borrow_mut())[i] = element;
        i.prefix_inc();
    }
    arr.write({ (*all.borrow()).clone() });
}
pub fn Consume_2(arr: std_unique_ptr_intarrarr__std_default_delete_intarrarr__, mut N: ()) -> () {
    let arr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(arr));
    let mut sum: () = 0_();
    let mut i: () = -1_();
    'loop_: while (i.prefix_inc() < N) {
        sum += { (*arr.borrow())[i] };
    }
    return sum;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut N: () = 10_();
    let arr: Value<std_unique_ptr_intarrarr__std_default_delete_intarrarr__> =
        Rc::new(RefCell::new(({ make_unique_1(N) })));
    ({ All_0(arr.as_pointer(), N, 1_()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
