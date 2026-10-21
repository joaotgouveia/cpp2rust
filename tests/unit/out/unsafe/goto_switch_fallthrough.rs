extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sm_0(mut n: ()) -> () {
    let mut ret: () = unimplemented!();
    goto_block!({
        '__entry: {
            ret = unimplemented!();
            switch!(match n {
                __v if __v == unimplemented!() => {
                    ret += 1_();
                }
                __v if __v == 1_() => {
                    ret += 10_();
                    goto!('out);
                }
                _ => {
                    ret += 100_();
                    break;
                }
            });
            ret += 1000_();
        }
        'out: {
            return ret;
        }
    });
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
