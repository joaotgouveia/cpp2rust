extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_nonnull_0(mut count: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut result: () = -1_();
    let mut i: () = 0_();
    'loop_: while ((((i) < (count)) as ()) != 0) {
        let mut p: *mut () = ap.arg::<*mut ()>();
        if (((!((p).is_null())) as ()) != 0) {
            result = (*p);
            break;
        }
        i.postfix_inc();
    }
    return result;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = 42_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
