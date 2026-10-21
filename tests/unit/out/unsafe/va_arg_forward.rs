extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn inner_0(mut count: (), mut ap: *const VaArg) -> () {
    let mut total: () = unimplemented!();
    let mut i: () = unimplemented!();
    'loop_: while ((((i) < (count)) as ()) != 0) {
        total += ap.arg::<()>();
        i.postfix_inc();
    }
    return total;
}
pub unsafe fn outer_1(mut count: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut result: () = (unsafe { inner_0(count, ap) });
    return result;
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
