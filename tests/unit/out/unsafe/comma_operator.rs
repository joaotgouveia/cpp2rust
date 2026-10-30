extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: (),
    pub second: (),
}
pub unsafe fn twice_0(v: *const ()) -> () {
    return ((*v) * (2_()));
}
pub unsafe fn sum_1(p: *const Pair) -> () {
    return (((*p).first) + ((*p).second));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 1_();
    let mut y: () = {
        x = 2_();
        ((x) + (1_()))
    };
    unimplemented!();
    unimplemented!();
    let mut z: () = {
        {
            1_();
            2_()
        };
        3_()
    };
    unimplemented!();
    let mut counter: () = 0_();
    let mut w: () = {
        {
            counter.postfix_inc();
            counter.postfix_inc()
        };
        counter
    };
    unimplemented!();
    unimplemented!();
    let mut a: () = 0_();
    let mut b: () = 0_();
    if {
        {
            a = 1_();
            b = 2_()
        };
        (((a) + (b)) > (0_()))
    } {
        unimplemented!();
        unimplemented!();
    }
    let mut v1: () = unimplemented!();
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p1: Pair = Pair {
        first: 1_(),
        second: 2_(),
    };
    let mut p2: Pair = {
        b = 6_();
        p1
    };
    p2.first = 10_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
