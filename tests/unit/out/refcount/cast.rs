extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type E = u32;
pub const E_A: E = 0;
pub const E_B: E = 1;
pub const E_C: E = 2;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut size: u64 = 1_u64;
    ({ cpp2rust_assert_fail_0((size == 1_u64)) });
    let mut ul: u64 = 5_u64;
    let mut s1: u64 = ul;
    ({ cpp2rust_assert_fail_0((s1 == 5_u64)) });
    ul = (7_u64 as u64);
    ({ cpp2rust_assert_fail_0((ul == 7_u64)) });
    let mut i: i32 = 2;
    let mut e: E = ((i) as E);
    ({ cpp2rust_assert_fail_0(((e as i32) == (E_C as i32))) });
    ({ cpp2rust_assert_fail_0(((e as i32) == 2)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
