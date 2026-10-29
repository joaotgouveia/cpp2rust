extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct A {
    #[offset(0)]
    pub v: (),
}
impl A {
    pub fn new_1() -> Self {
        Self { v: 1_() }
    }
    pub fn new_2(mut v: ()) -> Self {
        Self { v: v }
    }
}
impl Default for A {
    fn default() -> Self {
        { A::new_1() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct B {
    #[offset(0)]
    pub v: (),
}
impl B {
    pub fn new() -> Self {
        Self { v: 2_() }
    }
}
impl Default for B {
    fn default() -> Self {
        { B::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NoDefault {
    #[offset(0)]
    pub v: (),
}
impl NoDefault {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
}
pub fn used_0(x: Option<A>) -> () {
    let mut x: A = x.unwrap_or_else(|| A::new_1());
    return x.v;
}
pub fn used_1(x: Option<B>) -> () {
    let mut x: B = x.unwrap_or_else(|| B::new());
    return x.v;
}
pub fn scaled_2(mut x: A, n: Option<()>) -> () {
    let mut n: () = n.unwrap_or_else(|| 4usize);
    return (x.v * n);
}
pub fn always_given_3(mut x: NoDefault) -> () {
    return x.v;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S_NoDefault_ {
    #[offset(0)]
    pub v: (),
}
impl S_NoDefault_ {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let s: Value<S_NoDefault_> = Rc::new(RefCell::new(S_NoDefault_::new({ 1_() })));
    unimplemented!();
    return 0_();
}
pub trait S_NoDefault_Impl {
    fn get(&self, t: NoDefault) -> ();
}
impl S_NoDefault_Impl for Ptr<S_NoDefault_> {
    fn get(&self, mut t: NoDefault) -> () {
        return ((*self).with(|__s| __s.v) + t.v);
    }
}
pub fn __cpp2rust_init_globals() {}
