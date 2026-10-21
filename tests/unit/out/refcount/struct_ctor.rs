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
pub struct StructWithCtor {
    #[offset(0)]
    x1_: (),
    #[offset(4)]
    x2_: (),
}
impl StructWithCtor {
    pub fn new(mut x1: (), mut x2: ()) -> Self {
        let __this: Value<StructWithCtor> = Rc::new(RefCell::new(Self { x1_: x1, x2_: x2 }));
        let this: Ptr<StructWithCtor> = __this.as_pointer();
        field!(this, x1_).with_mut(|__v| __v.prefix_inc());
        field!(this, x2_).with_mut(|__v| __v.prefix_dec());
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn foo_0(x: Ptr<()>) -> Ptr<()> {
    return (x).clone();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Value_ {
    #[offset(0)]
    pub v: (),
}
impl Value_ {
    pub fn new(mut u: ()) -> Self {
        Self { v: u }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Ptr_ {
    #[offset(0)]
    #[byte_size(4)]
    pub v1: Value_,
    #[offset(4)]
    #[byte_size(4)]
    pub v2: Value_,
}
impl Ptr_ {
    pub fn new() -> Self {
        Self {
            v1: Value_::new({ 11_() }),
            v2: Value_::new({ 22_() }),
        }
    }
}
impl Default for Ptr_ {
    fn default() -> Self {
        { Ptr_::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let struct_with_ctor: Value<StructWithCtor> =
        Rc::new(RefCell::new(StructWithCtor::new({ 1_() }, { 2_() })));
    let x: Value<()> = Rc::new(RefCell::new(3_()));
    unimplemented!();
    let mut p: Ptr_ = Ptr_::new();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait StructWithCtorImpl {
    fn x1(&self) -> Ptr<()>;
    fn x2(&self) -> Ptr<()>;
}
impl StructWithCtorImpl for Ptr<StructWithCtor> {
    fn x1(&self) -> Ptr<()> {
        return field_ptr!((*self), x1_);
    }
    fn x2(&self) -> Ptr<()> {
        return field_ptr!((*self), x2_);
    }
}
pub fn __cpp2rust_init_globals() {}
