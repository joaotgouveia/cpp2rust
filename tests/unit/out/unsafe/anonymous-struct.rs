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
pub struct Outer_Named {
    pub a: (),
    pub b: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_0 {
    pub c: (),
    pub d: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub g: (),
    pub h: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_2 {
    pub e: (),
    pub f: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_4 {
    pub j: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_5 {
    pub k: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    pub i: (),
    pub inner_named: anon_4,
    pub anon_5: anon_5,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub named: Outer_Named,
    pub anonymous_named_0: anon_0,
    pub anonymous_named_1: anon_1,
    pub anon_2: anon_2,
    pub anon_3: anon_3,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut o: Outer = <Outer>::default();
    o.named.a = 1_();
    o.named.b = 2_();
    o.anonymous_named_0.c = 3_();
    o.anonymous_named_0.d = 4_();
    o.anonymous_named_1.g = 5_();
    o.anonymous_named_1.h = 6_();
    o.anon_2.e = 7_();
    o.anon_2.f = 8_();
    o.anon_3.i = 9_();
    o.anon_3.inner_named.j = 10_();
    o.anon_3.anon_5.k = 11_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut s: anon_6 = <anon_6>::default();
    s.x = 1_();
    s.z = 2_();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_6 {
    pub x: (),
    pub z: (),
}
pub unsafe fn __cpp2rust_init_globals() {}
