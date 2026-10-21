extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn first_0() -> () {
    let mut p: anon_1 = <anon_1>::default();
    p.x = 1_();
    p.y = 2_();
    return (p.x + p.y);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct anon_1 {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
pub fn second_2() -> () {
    let mut q: anon_3 = <anon_3>::default();
    q.a = 10_();
    q.b = 20_();
    return ((q.a + q.b) as ());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct anon_3 {
    #[offset(0)]
    pub a: (),
    #[offset(8)]
    pub b: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
