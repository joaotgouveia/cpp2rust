extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn while_in_switch_break_0(mut n: ()) -> () {
    let mut r: () = 0_();
    'switch: {
        match { n } {
            __v if __v == 0_() => {
                let mut i: () = 0_();
                'loop_: while ((i) < (10_())) {
                    if ((i) == (4_())) {
                        break;
                    }
                    r += i;
                    i.prefix_inc();
                }
                r += 1000_();
                break 'switch;
            }
            _ => {
                r = -1_();
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
