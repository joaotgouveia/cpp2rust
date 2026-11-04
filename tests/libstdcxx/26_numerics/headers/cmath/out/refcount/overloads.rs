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
    let mut i: i32 = -1_i32;
    let mut j: i32 = 9;
    let mut ans: f64 = 0_f64;
    ans = (i.abs() as f64);
    ans = ({ acos_0(i) });
    ans = ({ asin_1(i) });
    ans = ({ atan_2(i) });
    ans = ({ atan2_3(i, j) });
    ans = ({ cos_4(i) });
    ans = ({ cosh_5(i) });
    ans = ({ exp_6(i) });
    ans = ({ fabs_7(i) });
    ans = ({ floor_8(i) });
    ans = ({ log_9(i) });
    ans = ({ log10_10(i) });
    ans = ({ sqrt_11(i) });
    ans = ({ sin_12(i) });
    ans = ({ sinh_13(j) });
    ans = ({ tan_14(i) });
    ans = ({ tanh_15(i) });
    ans = { ans };
    0
}
pub fn __cpp2rust_init_globals() {}
