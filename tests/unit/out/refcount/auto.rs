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
    let mut x1: i32 = 1;
    let mut x2: i16 = 2_i16;
    let mut x3: u32 = 4_u32;
    let v: Value<std_vector_int__std_allocator_int__> = Rc::new(RefCell::new(
        <std_vector_int__std_allocator_int__>::default(),
    ));
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(1));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    ({
        let ___x: Value<i32> = Rc::new(RefCell::new(2));
        (*v.borrow()).push_back_60(___x.as_pointer())
    });
    let mut sum: i32 = 0;
    'loop_: for elem in IterRange::new(({ (*v.borrow()).begin_28() }), ({ (*v.borrow()).end_30() }))
    {
        let mut elem: i32 = (elem.read());
        sum += elem;
    }
    ({ cpp2rust_assert_fail_0((sum == 3)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
