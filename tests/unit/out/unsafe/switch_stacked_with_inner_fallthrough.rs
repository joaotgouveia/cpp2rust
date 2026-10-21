extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn stacked_with_inner_fallthrough_0(mut x: (), mut flag: ()) -> () {
    let mut r: () = 0_();
    switch!(match x {
        __v if __v == 1_() || __v == 2_() || __v == 3_() => {
            if !(flag != 0) {
                r = 50_();
                break;
            };
        }
        _ => {
            r = 999_();
            break;
        }
    });
    return r;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
