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
pub struct pair {
    pub a: (),
    pub b: (),
}
pub unsafe fn sum_mixed_0(mut count: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut total: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((((i) < (count)) as ()) != 0) {
        let mut tag: () = ap.arg::<()>();
        if ((((tag) == (0_())) as ()) != 0) {
            total += ap.arg::<()>();
        } else if ((((tag) == (1_())) as ()) != 0) {
            total += (ap.arg::<()>() as ());
        } else if ((((tag) == (3_())) as ()) != 0) {
            let mut p: pair = ap.arg::<pair>();
            total += ((p.a) * (p.b));
        } else {
            let mut val: () = ap.arg::<()>();
            total += (val as ());
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: pair = pair { a: 7_(), b: 8_() };
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
