extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn continue_inside_switch_0(mut n: ()) -> () {
    let mut r: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((i) < (n)) {
        'switch: {
            match { i } {
                __v if __v == unimplemented!() || __v == 2_() || __v == 4_() => {
                    i.prefix_inc();
                    continue 'loop_;
                }
                _ => {
                    r += i;
                    break 'switch;
                }
            }
        };
        r += 1000_();
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
