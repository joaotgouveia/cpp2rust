extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn empty_case_with_break_0(mut x: ()) -> () {
    let mut r: () = 5_();
    'switch: {
        match { x } {
            __v if __v == 1_() => {
                break 'switch;
            }
            __v if __v == 2_() => {
                r = 2_();
                break 'switch;
            }
            _ => {
                r = 9_();
                break 'switch;
            }
        }
    };
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
