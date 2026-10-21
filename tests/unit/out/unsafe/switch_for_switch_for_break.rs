extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn for_switch_for_break_0(mut n: ()) -> () {
    let mut r: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (n)) {
        'switch: {
            match { i } {
                __v if __v == 1_() => {
                    let mut j: () = unimplemented!();
                    'loop_: while ((j) < (10_())) {
                        if ((j) == (2_())) {
                            break;
                        }
                        r += 1_();
                        j.prefix_inc();
                    }
                    r += 100_();
                    break 'switch;
                }
                _ => {
                    r += 10_();
                    break 'switch;
                }
            }
        };
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
