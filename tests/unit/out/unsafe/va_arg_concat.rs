extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sum_ints_0(mut first: (), __args: Box<[VaArg]>) -> () {
    let mut args: *const VaArg = ::std::ptr::null();
    let mut total: () = first;
    args = __args.as_ptr();
    let mut val: () = unimplemented!();
    'loop_: while (((({
        val = args.arg::<()>();
        val
    }) != (0_())) as ())
        != 0)
    {
        total += val;
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
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
