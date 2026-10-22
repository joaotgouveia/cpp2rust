extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn array_ref_0(a: Ptr<()>) -> () {
    elem!((a), 0_()).write({ (elem!((a), 0_()).read()).wrapping_add(1_()) });
    return (elem!((a), (3_() as ()).wrapping_sub(1_())).read());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PtrCtor_unsigned_long_ {
    #[offset(0)]
    pub v: (),
}
impl PtrCtor_unsigned_long_ {
    pub fn new(mut p: Ptr<()>) -> Self {
        Self {
            v: (elem!(p, 1_()).read()),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct RefCtor_unsigned_long_ {
    #[offset(0)]
    pub v: (),
}
impl RefCtor_unsigned_long_ {
    pub fn new(x: Ptr<()>) -> Self {
        Self {
            v: (x.read()).wrapping_add(1_()),
        }
    }
}
pub fn neg_ll_1() -> () {
    return -9000000_();
}
pub fn neg_char_2() -> () {
    return (b'\x80' as ());
}
pub fn neg_int_3() -> () {
    return -1_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a1: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_()])));
    unimplemented!();
    unimplemented!();
    let a2: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([4_(), 5_()])));
    let mut pc: PtrCtor_unsigned_long_ =
        PtrCtor_unsigned_long_::new({ (a2.as_pointer() as Ptr<()>) });
    unimplemented!();
    let v1: Value<()> = Rc::new(RefCell::new(6_()));
    let mut rc: RefCtor_unsigned_long_ = RefCtor_unsigned_long_::new({ v1.as_pointer() });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
