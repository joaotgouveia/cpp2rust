extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn identity_0(mut x: ()) -> () {
    return x;
}
pub unsafe fn apply_1(mut x: (), mut fn_: Option<Option<unsafe fn(()) -> ()>>) -> () {
    let mut fn_: Option<unsafe fn(()) -> ()> = fn_.unwrap_or_else(|| unsafe { None });
    if !(fn_).is_none() {
        return (unsafe { (fn_).unwrap()(x) });
    }
    return x;
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
    let mut negate: Option<unsafe fn(()) -> ()> = Some(|mut x: ()| -> () {
        unsafe {
            return -x;
        }
    });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
