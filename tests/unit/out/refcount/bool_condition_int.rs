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
    let mut n: i32 = 3;
    let mut zero: i32 = 0;
    let mut u: u32 = 4_u32;
    let mut ul: u64 = 5_u64;
    let mut ll: i64 = 6_i64;
    let mut ch: i8 = ('a' as i8);
    if (n != 0) {
        ({ cpp2rust_assert_fail_0(true) });
    }
    if !(n != 0) {
        ({ cpp2rust_assert_fail_0(false) });
    }
    if (zero != 0) {
        ({ cpp2rust_assert_fail_0(false) });
    }
    if !(zero != 0) {
        ({ cpp2rust_assert_fail_0(true) });
    }
    if (u != 0) {
        ({ cpp2rust_assert_fail_0(true) });
    }
    if (ul != 0) {
        ({ cpp2rust_assert_fail_0(true) });
    }
    if (ll != 0) {
        ({ cpp2rust_assert_fail_0(true) });
    }
    if (ch != 0) {
        ({ cpp2rust_assert_fail_0(true) });
    }
    let mut loop_count: i32 = 0;
    let mut counter: i32 = 3;
    'loop_: while (counter != 0) {
        counter.prefix_dec();
        loop_count.prefix_inc();
    }
    ({ cpp2rust_assert_fail_0((loop_count == 3)) });
    let mut i: i32 = 5;
    'loop_: while (i != 0) {
        loop_count.prefix_inc();
        i.prefix_dec();
    }
    ({ cpp2rust_assert_fail_0((loop_count == 8)) });
    let mut t: i32 = if (n != 0) { 100 } else { 200 };
    ({ cpp2rust_assert_fail_0((t == 100)) });
    let mut t2: i32 = if (zero != 0) { 100 } else { 200 };
    ({ cpp2rust_assert_fail_0((t2 == 200)) });
    let mut t7: i32 = (!(n != 0) as i32);
    ({ cpp2rust_assert_fail_0((t7 == 0)) });
    let mut t8: i32 = (!(zero != 0) as i32);
    ({ cpp2rust_assert_fail_0((t8 == 1)) });
    let mut b1: bool = (n != 0);
    ({ cpp2rust_assert_fail_0(b1) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
