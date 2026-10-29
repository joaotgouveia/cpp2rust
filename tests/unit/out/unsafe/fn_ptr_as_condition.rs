extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn double_it_0(mut x: *mut ()) {
    (*x) *= 2_();
}
pub unsafe fn maybe_call_1(mut cb: Option<unsafe fn(*mut ())>, mut x: *mut ()) {
    if !(cb).is_none() {
        (unsafe { (cb).unwrap()(x) });
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: () = 5_();
    (unsafe { maybe_call_1(Some(double_it_0), (&mut a as *mut ())) });
    unimplemented!();
    let mut b: () = 5_();
    (unsafe { maybe_call_1(None, (&mut b as *mut ())) });
    unimplemented!();
    let mut fn_: Option<unsafe fn(*mut ())> = None;
    if !(!(fn_).is_none()) {
        fn_ = Some(double_it_0);
    }
    let mut c: () = 3_();
    if !(fn_).is_none() {
        (unsafe { (fn_).unwrap()((&mut c as *mut ())) });
    }
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
