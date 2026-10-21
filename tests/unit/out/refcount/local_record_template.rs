extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn get_0(mut t: Local_1) -> () {
    return t.x;
}
pub fn get_2(mut t: Local_3) -> () {
    return t.x;
}
pub fn get_4(mut t: Local_5) -> () {
    return t.x;
}
pub fn get_6(mut t: Local_7) -> () {
    return t.x;
}
pub fn twice_8(mut t: Local_3) -> () {
    return (t.x * 2_());
}
pub fn wrap_9(mut v: ()) -> () {
    let mut l: Local_5 = Local_5 { x: v };
    return ({ get_4(l) });
}
pub fn wrap_10(mut v: ()) -> () {
    let mut l: Local_7 = Local_7 { x: v };
    return ({ get_6(l) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_5 {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Local_7 {
    #[offset(0)]
    pub x: (),
}
pub fn other_11() -> () {
    let mut l: Local_1 = Local_1 { x: 3_(), y: 4_() };
    return (({ get_0((l).clone()) }) + l.y);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Local_1 {
    #[offset(0)]
    pub x: (),
    #[offset(8)]
    pub y: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut l: Local_3 = Local_3 { x: 7_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Local_3 {
    #[offset(0)]
    pub x: (),
}
pub fn __cpp2rust_init_globals() {}
