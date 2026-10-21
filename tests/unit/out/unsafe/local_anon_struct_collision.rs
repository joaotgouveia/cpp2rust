extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn first_0() -> () {
    let mut p: anon_1 = <anon_1>::default();
    p.x = 1_();
    p.y = 2_();
    return ((p.x) + (p.y));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub x: (),
    pub y: (),
}
pub unsafe fn second_2() -> () {
    let mut q: anon_3 = <anon_3>::default();
    q.a = 10_();
    q.b = 20_();
    return (((q.a) + (q.b)) as ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    pub a: (),
    pub b: (),
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
    return unimplemented!();
}
pub unsafe fn __cpp2rust_init_globals() {}
