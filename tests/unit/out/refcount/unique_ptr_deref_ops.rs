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
    let p: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(10));
            make_unique_0(___args.as_pointer())
        }),
    ));
    {
        (*p.borrow_mut()).with_mut(|__v| *__v = *__v + 5)
    };
    {
        (*p.borrow_mut()).with_mut(|__v| *__v = *__v - 3)
    };
    {
        (*p.borrow_mut()).with_mut(|__v| *__v = *__v * 2)
    };
    let q: Value<std_unique_ptr_int__std_default_delete_int__> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(1));
            make_unique_0(___args.as_pointer())
        }),
    ));
    let mut sum: i32 = (((*p.borrow()).read()) + ((*q.borrow()).read()));
    ({ cpp2rust_assert_fail_1((sum == 25)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
