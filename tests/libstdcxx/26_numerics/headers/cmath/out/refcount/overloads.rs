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
    ans = (({ abs_0(i) }) as f64);
    ans = ({ acos_1(i) });
    ans = ({ asin_2(i) });
    ans = ({ atan_3(i) });
    ans = ({ atan2_4(i, j) });
    ans = ({ cos_5(i) });
    ans = ({ cosh_6(i) });
    ans = ({ exp_7(i) });
    ans = ({ fabs_8(i) });
    ans = ({ floor_9(i) });
    ans = ({ log_10(i) });
    ans = ({ log10_11(i) });
    ans = ({ sqrt_12(i) });
    ans = ({ sin_13(i) });
    ans = ({ sinh_14(j) });
    ans = ({ tan_15(i) });
    ans = ({ tanh_16(i) });
    ans = { ans };
    0
}
pub fn __cpp2rust_init_globals() {}
