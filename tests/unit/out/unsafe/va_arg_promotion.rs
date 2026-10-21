extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_promotions_0(mut count: (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut a: () = ap.arg::<()>();
    let mut b: () = ap.arg::<()>();
    let mut c: () = ap.arg::<()>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return (((a) + (b)) + (c as ()));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut x: () = ('A' as ());
    let mut y: () = 10_();
    let mut z: () = 3_f32;
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
