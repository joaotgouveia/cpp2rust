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
pub struct Triple {
    pub a: (),
    pub b: (),
    pub c: (),
}
pub unsafe fn sum_0(t: *const Triple) -> () {
    let __decomp_1: *const Triple = t;
    return ((((*__decomp_1).a) + (if (*__decomp_1).b { 1_() } else { 0_() })) + ((*__decomp_1).c));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut t: Triple = Triple {
        a: 10_(),
        b: false,
        c: 20_(),
    };
    let __decomp_2: *mut Triple = &mut t;
    (*__decomp_2).a = 11_();
    (*__decomp_2).b = true;
    (*__decomp_2).c += 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    t.a = 12_();
    unimplemented!();
    let __decomp_3: *const Triple = &t;
    t.c = 30_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
