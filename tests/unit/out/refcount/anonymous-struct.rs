extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_Named {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_0 {
    #[offset(0)]
    pub c: (),
    #[offset(4)]
    pub d: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    pub g: (),
    #[offset(4)]
    pub h: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_2 {
    #[offset(0)]
    pub e: (),
    #[offset(4)]
    pub f: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_4 {
    #[offset(0)]
    pub j: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct anon_5 {
    #[offset(0)]
    pub k: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct anon_3 {
    #[offset(0)]
    pub i: (),
    #[offset(4)]
    #[byte_size(4)]
    pub inner_named: anon_4,
    #[offset(8)]
    #[byte_size(4)]
    pub anon_5: anon_5,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(44)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(8)]
    pub named: Outer_Named,
    #[offset(8)]
    #[byte_size(8)]
    pub anonymous_named_0: anon_0,
    #[offset(16)]
    #[byte_size(8)]
    pub anonymous_named_1: anon_1,
    #[offset(24)]
    #[byte_size(8)]
    pub anon_2: anon_2,
    #[offset(32)]
    #[byte_size(12)]
    pub anon_3: anon_3,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
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
    return 0_();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_6 {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub z: (),
}
pub fn __cpp2rust_init_globals() {}
