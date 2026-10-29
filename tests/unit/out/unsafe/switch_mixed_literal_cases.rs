extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn mixed_literal_cases_0(mut x: ()) -> () {
    'switch: {
        match { x } {
            __v if __v == -1_() => {
                return 1_();
            }
            __v if __v == 16_() => {
                return 2_();
            }
            __v if __v == 65152_() => {
                return 3_();
            }
            __v if __v == -255_() => {
                return 4_();
            }
            _ => {
                return 0_();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
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
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
