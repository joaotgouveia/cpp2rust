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
    let storage: Value<i32> = Rc::new(RefCell::new(7));
    let mut p: Ptr<i32> = (storage.as_pointer());
    let mut np: Ptr<i32> = Ptr::<i32>::null();
    if !(p).is_null() {
        ({ cpp2rust_assert_fail_0(true) });
    }
    if !(!(p).is_null()) {
        ({ cpp2rust_assert_fail_0(false) });
    }
    if !(np).is_null() {
        ({ cpp2rust_assert_fail_0(false) });
    }
    if !(!(np).is_null()) {
        ({ cpp2rust_assert_fail_0(true) });
    }
    let mut iter: Ptr<i32> = (p).clone();
    let mut iters: i32 = 0;
    'loop_: while !(iter).is_null() {
        iters.prefix_inc();
        iter = Ptr::<i32>::null();
    }
    ({ cpp2rust_assert_fail_0((iters == 1)) });
    let mut t3: i32 = if !(p).is_null() { 1 } else { 0 };
    ({ cpp2rust_assert_fail_0((t3 == 1)) });
    let mut t4: i32 = if !(np).is_null() { 1 } else { 0 };
    ({ cpp2rust_assert_fail_0((t4 == 0)) });
    let mut t5: i32 = (!(!(p).is_null()) as i32);
    ({ cpp2rust_assert_fail_0((t5 == 0)) });
    let mut t6: i32 = (!(!(np).is_null()) as i32);
    ({ cpp2rust_assert_fail_0((t6 == 1)) });
    let mut b2: bool = !(p).is_null();
    let mut b3: bool = !(np).is_null();
    ({ cpp2rust_assert_fail_0(b2) });
    ({ cpp2rust_assert_fail_0(!(b3)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
