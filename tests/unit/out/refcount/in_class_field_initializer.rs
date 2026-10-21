extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
impl Default for Inner {
    fn default() -> Self {
        { Inner { x: 3_(), y: 4_() } }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct S {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
    #[offset(8)]
    #[byte_size(8)]
    pub c: Inner,
    #[offset(16)]
    #[byte_size(8)]
    pub d: Inner,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                a: 1_(),
                b: 2_(),
                c: <Inner>::default(),
                d: <Inner>::default(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Boxed_int_ {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    pub tag: (),
}
impl Boxed_int_ {
    pub fn new(mut x: (), mut t: ()) -> Self {
        Self { v: x, tag: t }
    }
}
impl Default for Boxed_int_ {
    fn default() -> Self {
        {
            Boxed_int_ {
                v: unimplemented!(),
                tag: unimplemented!(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut s: S = <S>::default();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut boxed: Boxed_int_ = Boxed_int_::new({ 5_() }, { 9_() });
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
