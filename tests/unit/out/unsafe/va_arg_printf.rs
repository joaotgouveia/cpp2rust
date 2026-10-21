extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn logf_impl_0(mut fmt: *const (), mut ap: *const VaArg) -> () {
    &(fmt);
    return ((ap.arg::<()>()) + (ap.arg::<()>()));
}
pub unsafe fn logf_1(mut fmt: *const (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut result: () = (unsafe { logf_impl_0(fmt, ap) });
    return result;
}
pub unsafe fn lenf_2(mut fmt: *const (), __args: Box<[VaArg]>) -> () {
    let mut ap: *const VaArg = ::std::ptr::null();
    ap = __args.as_ptr();
    let mut s: *const () = ap.arg::<*const ()>();
    let mut result: () = ((unsafe { strlen_3(s) }) as ());
    return result;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut dummy: *const () = (c"dummy".as_ptr().cast_mut()).cast_const();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
