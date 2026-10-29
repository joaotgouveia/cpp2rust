extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sm_0(mut n: ()) -> () {
    let mut steps: () = 0_();
    switch!(match n {
        __v if __v == 0_() => 'target: {
            steps += 1_();
            break;
        }
        __v if __v == 1_() => {
            steps += 10_();
            goto!('target);
        }
        _ => {
            steps = -1_();
            break;
        }
    });
    return steps;
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
