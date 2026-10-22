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
pub struct S {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl S {
    pub fn new(mut a: (), mut b: ()) -> Self {
        Self { a: a, b: b }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut s1: S = S::new({ 1_() }, { 2_() });
    let s2: Ptr<S> = unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
