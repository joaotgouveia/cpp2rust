extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(mut x: i32) -> i32 {
    return (x + 1);
}
pub fn dec_1(mut x: i32) -> i32 {
    return (x - 1);
}
pub fn pick_2(mut choose_inc: i32) -> FnPtr<fn(i32) -> i32> {
    if (choose_inc != 0) {
        return FnPtr::<fn(i32) -> i32>::new(inc_0);
    }
    return FnPtr::<fn(i32) -> i32>::new(dec_1);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut f: FnPtr<fn(i32) -> i32> = ({ pick_2(1) });
    ({ cpp2rust_assert_fail_3(!((f).is_null())) });
    ({ cpp2rust_assert_fail_3(({ (f).clone() } == { FnPtr::<fn(i32) -> i32>::new(inc_0) })) });
    ({ cpp2rust_assert_fail_3((({ f.call(10) }) == 11)) });
    let mut g: FnPtr<fn(i32) -> i32> = ({ pick_2(0) });
    ({ cpp2rust_assert_fail_3(({ (g).clone() } == { FnPtr::<fn(i32) -> i32>::new(dec_1) })) });
    ({ cpp2rust_assert_fail_3((({ g.call(10) }) == 9)) });
    ({ cpp2rust_assert_fail_3(({ (f).clone() } != { (g).clone() })) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
