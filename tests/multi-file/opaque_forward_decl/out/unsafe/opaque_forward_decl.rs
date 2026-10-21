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
pub struct container {
    pub p: *mut opaque,
    pub x: (),
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut c: container = container {
        p: std::ptr::null_mut(),
        x: 42_(),
    };
    (unsafe { touch_0((&mut c as *mut container)) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn touch_0(mut c: *mut container) {
    &((*c).p);
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct opaque;
pub unsafe fn __cpp2rust_init_globals() {}
