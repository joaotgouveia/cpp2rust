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
pub struct Point {
    pub x: (),
    pub y: (),
}
pub unsafe fn sum_0(mut p: Point) -> () {
    return ((p.x) + (p.y));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut p: std_unique_ptr_Point__std_default_delete_Point__ = unimplemented!();
    (*unimplemented!()).x += 10_();
    (*unimplemented!()).y = (((*unimplemented!()).x) + ((*unimplemented!()).y));
    let mut s: () = (unsafe { sum_0((*unimplemented!())) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
