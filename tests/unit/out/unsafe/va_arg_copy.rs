extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_with_copy_0(mut count: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    let mut aq: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    aq = ap.clone();
    let mut sum1: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((((i) < (count)) as ()) != 0) {
        sum1 += ap.arg::<()>();
        i.postfix_inc();
    }
    let mut sum2: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((((i) < (count)) as ()) != 0) {
        sum2 += aq.arg::<()>();
        i.postfix_inc();
    }
    unimplemented!();
    return ((sum1) + (sum2));
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
