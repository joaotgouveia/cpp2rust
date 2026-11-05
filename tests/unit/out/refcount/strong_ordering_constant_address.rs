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
    let e: Ptr<std_strong_ordering> = equal_0.with(|v| v.as_pointer());
    let g: Ptr<std_strong_ordering> = greater_1.with(|v| v.as_pointer());
    ({ cpp2rust_assert_fail_2(({ (e).clone() } == { (equal_0.with(|v| v.as_pointer())) })) });
    ({ cpp2rust_assert_fail_2(({ (g).clone() } == { (greater_1.with(|v| v.as_pointer())) })) });
    ({ cpp2rust_assert_fail_2(({ (e).clone() } != { (g).clone() })) });
    ({
        cpp2rust_assert_fail_2(
            ((less_3.with(|v| v.as_pointer())) != (equivalent_4.with(|v| v.as_pointer()))),
        )
    });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
