extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_then_product_0(mut first: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    let mut sum: () = first;
    let mut product: () = first;
    ap = __args.as_ptr();
    let mut val: () = unimplemented!();
    'loop_: while (((({
        val = ap.arg::<()>();
        val
    }) != (unimplemented!())) as ())
        != 0)
    {
        sum += val;
    }
    ap = __args.as_ptr();
    'loop_: while (((({
        val = ap.arg::<()>();
        val
    }) != (unimplemented!())) as ())
        != 0)
    {
        product *= val;
    }
    return ((sum) + (product));
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
