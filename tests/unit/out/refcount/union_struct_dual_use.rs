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
pub struct Inner {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
pub fn sum_inner_0(mut i: Ptr<Inner>) -> () {
    return ({ i.with(|__s| __s.a) } + { i.with(|__s| __s.b) });
}
#[derive(VaArg, FnPtrArg, ByteRepr, Clone)]
#[byte_size(16)]
pub struct anon_1 {
    #[offset(0)]
    #[byte_size(16)]
    __bytes: Box<[u8]>,
}
impl anon_1 {
    pub fn inner(this: Ptr<Self>) -> Ptr<Inner> {
        this.reinterpret_cast()
    }
    pub fn raw_(this: Ptr<Self>) -> Ptr<()> {
        this.reinterpret_cast()
    }
}
impl Default for anon_1 {
    fn default() -> Self {
        anon_1 {
            __bytes: Box::from([0u8; 16]),
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(16)]
    pub u: anon_1,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let standalone: Value<Inner> = <Value<Inner>>::default();
    (*standalone.borrow_mut()).a = 3_();
    (*standalone.borrow_mut()).b = 4_();
    unimplemented!();
    let outer: Value<Outer> = <Value<Outer>>::default();
    ({
        let ___s: () = ((outer.as_pointer()) as Ptr<Outer>).to_any();
        let ___n: () = 16usize;
        memset_2(___s, 0_(), ___n)
    });
    field!(anon_1::inner(field_ptr!(outer.as_pointer(), u)), a).write(3_());
    field!(anon_1::inner(field_ptr!(outer.as_pointer(), u)), b).write(4_());
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
