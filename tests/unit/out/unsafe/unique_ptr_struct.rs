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
    let mut p: std_unique_ptr_Point__std_default_delete_Point__ = (unsafe {
        let mut ___args: Point = Point { x: 3_(), y: 4_() };
        make_unique_1(&mut ___args)
    });
    (*p).x += 10_();
    (*p).y = (((*p).x) + ((*p).y));
    let mut s: () = (unsafe { sum_0((*p)) });
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
