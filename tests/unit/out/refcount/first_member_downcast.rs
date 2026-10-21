extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct base {
    #[offset(0)]
    pub kind: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct derived {
    #[offset(0)]
    #[byte_size(4)]
    pub head: base,
    #[offset(8)]
    pub value: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut d: Ptr<derived> = unimplemented!().reinterpret_cast::<derived>();
    unimplemented!();
    field!(field!(d, head), kind).write(3_());
    field!(d, value).write(7_());
    let mut b: Ptr<base> = (field_ptr!(d, head));
    let mut back: Ptr<derived> = b.reinterpret_cast::<derived>();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    field!(back, value).write(8_());
    unimplemented!();
    field!(b, kind).write(4_());
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
