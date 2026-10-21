extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn fallthrough_default_0(mut x: (), mut flag: ()) -> () {
    let mut r: () = unimplemented!();
    switch!(match x {
        __v if __v == 7_() => {
            if (flag != 0) {
                r = 100_();
                break;
            };
        }
        _ => {
            r = 42_();
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
