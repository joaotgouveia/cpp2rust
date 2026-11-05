extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn square_0(mut x: i32) -> i32 {
    return (x * x);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut ce: bool = true;
    ({ cpp2rust_assert_fail_1(ce) });
    let mut rt: bool = ({ __builtin_is_constant_evaluated_2() });
    ({ cpp2rust_assert_fail_1(!(rt)) });
    let mut n: i32 = -1_i32;
    ({ cpp2rust_assert_fail_1((n == -1_i32)) });
    let mut d: f64 = 5_f64;
    ({ cpp2rust_assert_fail_1((d == 5_f64)) });
    let mut big: u64 = 1099511627776_u64;
    ({ cpp2rust_assert_fail_1((big == 1099511627776_u64)) });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
