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
pub struct S {
    pub a: (),
    pub b: (),
    pub c: (),
}
pub unsafe fn bump_0(mut s: *mut S) -> () {
    (*s).b += 10_();
    return (*s).b;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: *mut S = (unimplemented!() as *mut S);
    unimplemented!();
    (*s).b = 1_();
    (*s).a = (unsafe { bump_0(s) });
    unimplemented!();
    unimplemented!();
    (*s).a = 1_();
    (*s).b = 2_();
    (*s).c = 0_();
    if (((*s).a) < ((*s).b)) && (((*s).c.postfix_inc()) == (0_())) {
        (*s).a = 5_();
    }
    unimplemented!();
    if (((*s).a) < ((*s).b)) && (((*s).c.postfix_inc()) == (0_())) {
        (*s).a = 6_();
    }
    unimplemented!();
    let mut x: () = (((*s).a)
        + ({
            (*s).b = 3_();
            (*s).b
        }));
    unimplemented!();
    let mut y: () = 0_();
    (*s).c = ({
        y = 99_();
        y
    });
    unimplemented!();
    (*s).a += (unsafe { bump_0(s) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
