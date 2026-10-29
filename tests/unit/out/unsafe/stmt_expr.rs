extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = {
        let mut a: () = 1_();
        let mut b: () = 2_();
        ((a) + (b))
    };
    unimplemented!();
    let mut counter: () = 0_();
    let mut y: () = {
        counter.postfix_inc();
        ((counter) * (10_()))
    };
    unimplemented!();
    unimplemented!();
    let mut z: () = {
        let mut v: () = 5_();
        if ((v) > (0_())) {
            v = ((v) * (2_()));
        }
        v
    };
    unimplemented!();
    unimplemented!();
    {
        counter.postfix_inc()
    };
    {
        if ((counter) != (2_())) {
            counter = 2_();
        }
    };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
