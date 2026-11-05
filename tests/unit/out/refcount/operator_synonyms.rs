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
    let mut a: u32 = 12_u32;
    let mut b: u32 = 10_u32;
    let mut z: u32 = 0_u32;
    ({ cpp2rust_assert_fail_0(!(z != 0)) });
    ({ cpp2rust_assert_fail_0((a != 0) && (b != 0)) });
    ({ cpp2rust_assert_fail_0(!((a != 0) && (z != 0))) });
    ({ cpp2rust_assert_fail_0((a != 0) || (z != 0)) });
    ({ cpp2rust_assert_fail_0((a != b)) });
    ({ cpp2rust_assert_fail_0(((!a) == !12_u32)) });
    ({ cpp2rust_assert_fail_0(((a & b) == 8_u32)) });
    ({ cpp2rust_assert_fail_0(((a | b) == 14_u32)) });
    ({ cpp2rust_assert_fail_0(((a ^ b) == 6_u32)) });
    a &= b;
    ({ cpp2rust_assert_fail_0((a == 8_u32)) });
    a |= b;
    ({ cpp2rust_assert_fail_0((a == 10_u32)) });
    a ^= b;
    ({ cpp2rust_assert_fail_0((a == 0_u32)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
