extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn negate_0(mut x: *mut ()) {
    (*x) = -(*x);
}
pub unsafe fn zero_out_1(mut x: *mut ()) {
    (*x) = unimplemented!();
}
pub unsafe fn run_2(mut fn_: Option<unsafe fn(*mut ())>, mut x: *mut ()) {
    (unsafe { (fn_).unwrap()(x) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: () = 42_();
    (unsafe { run_2(Some(negate_0), (&mut a as *mut ())) });
    unimplemented!();
    (unsafe { run_2(Some(zero_out_1), (&mut a as *mut ())) });
    unimplemented!();
    let mut fn_: Option<unsafe fn(*mut ())> = Some(negate_0);
    unimplemented!();
    let mut b: () = 10_();
    (unsafe { (fn_).unwrap()((&mut b as *mut ())) });
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
