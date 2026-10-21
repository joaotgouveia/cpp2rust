extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn switch_in_loop_0(mut n: ()) -> () {
    let mut r: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (n)) {
        'switch: {
            match { ((i) % (3_())) } {
                __v if __v == unimplemented!() => {
                    r += 1_();
                    break 'switch;
                }
                __v if __v == 1_() => {
                    r += 2_();
                    break 'switch;
                }
                _ => {
                    r += 3_();
                    break 'switch;
                }
            }
        };
        r += 10_();
        i.prefix_inc();
    }
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
