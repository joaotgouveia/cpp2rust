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
pub struct Pod {
    pub v: (),
}
pub unsafe fn zero_0() -> *mut () {
    return std::ptr::null_mut();
}
pub unsafe fn zero_1() -> () {
    return unimplemented!();
}
pub unsafe fn destroy_2(mut p: *mut ()) {}
pub unsafe fn destroy_3(mut p: *mut Pod) {}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut i: () = unimplemented!();
    let mut d: () = unimplemented!();
    let mut p: *mut () = (unsafe { zero_0() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut x: () = 5_();
    (unsafe { destroy_2((&mut x as *mut ())) });
    unimplemented!();
    let mut pod: Pod = Pod { v: 7_() };
    (unsafe { destroy_3((&mut pod as *mut Pod)) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
