extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn mixed_args_0(mut count: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut total: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((((i) < (count)) as ()) != 0) {
        let mut tag: () = ap.arg::<()>();
        if ((((tag) == (unimplemented!())) as ()) != 0) {
            total += ap.arg::<()>();
        } else {
            let mut ptr: *mut () = ap.arg::<*mut ()>();
            total += (*ptr);
        }
        i.postfix_inc();
    }
    return total;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 100_();
    unimplemented!();
    let mut y: () = 50_();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
