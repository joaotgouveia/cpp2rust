extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i: () = -1_();
    let mut j: () = 9_();
    let mut ans: () = unimplemented!();
    ans = (unsafe { abs_0(i) });
    ans = (unsafe { acos_1(i) });
    ans = (unsafe { asin_2(i) });
    ans = (unsafe { atan_3(i) });
    ans = unimplemented!();
    ans = (unsafe { cos_4(i) });
    ans = (unsafe { cosh_5(i) });
    ans = (unsafe { exp_6(i) });
    ans = (unsafe { fabs_7(i) });
    ans = (unsafe { floor_8(i) });
    ans = (unsafe { log_9(i) });
    ans = (unsafe { log10_10(i) });
    ans = (unsafe { sqrt_11(i) });
    ans = (unsafe { sin_12(i) });
    ans = (unsafe { sinh_13(j) });
    ans = (unsafe { tan_14(i) });
    ans = (unsafe { tanh_15(i) });
    ans = ans;
    0
}
pub unsafe fn __cpp2rust_init_globals() {}
