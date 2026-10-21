extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn no_default_0(mut x: ()) -> () {
    let mut r: () = -1_();
    'switch: {
        match { x } {
            __v if __v == 7_() => {
                r = 1_();
                break 'switch;
            }
            __v if __v == 8_() => {
                r = 2_();
                break 'switch;
            }
            _ => {}
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
