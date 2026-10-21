extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn extract_nth_0(mut n: (), mut ap: *const VaArg) -> () {
    let mut i: () = unimplemented!();
    'loop_: while ((((i) < (n)) as ()) != 0) {
        ap.arg::<()>();
        i.postfix_inc();
    }
    return ap.arg::<()>();
}
pub unsafe fn middle_layer_1(mut n: (), mut ap: *const VaArg) -> () {
    return (unsafe { extract_nth_0(n, ap) });
}
pub unsafe fn top_level_2(mut n: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut result: () = (unsafe { middle_layer_1(n, ap) });
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
