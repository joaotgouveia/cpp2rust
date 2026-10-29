extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn default_then_case_0(mut x: ()) -> () {
    let mut r: () = 0_();
    'switch: {
        match { x } {
            __v if __v == 1_() => {
                r = 1_();
                break 'switch;
            }
            __v if __v == 3_() => {
                r = 3_();
                break 'switch;
            }
            _ => {
                r = 77_();
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
