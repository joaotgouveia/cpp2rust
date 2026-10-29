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
pub struct In {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct S {
    #[offset(0)]
    #[byte_size(8)]
    pub in_: In,
    #[offset(8)]
    pub total: (),
    #[offset(12)]
    pub n: (),
    #[offset(16)]
    #[byte_size(16)]
    pub arr: Box<[()]>,
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                in_: <In>::default(),
                total: unimplemented!(),
                n: unimplemented!(),
                arr: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Node {
    #[offset(0)]
    pub x: (),
    #[offset(8)]
    #[byte_size(8)]
    pub self_: Ptr<Node>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p: Ptr<S> = unimplemented!().reinterpret_cast::<S>();
    unimplemented!();
    let mut q: Ptr<S> = (p).clone();
    field!(field!(p, in_), x).write(1_());
    field!(field!(p, in_), y).write(2_());
    field!(p, total).write(({ q.with(|__s| __s.in_.x) } + { q.with(|__s| __s.in_.y) }));
    unimplemented!();
    let mut ip: Ptr<In> = (field_ptr!(p, in_));
    field!(ip, x).write((p.with(|__s| __s.total) + 1_()));
    unimplemented!();
    array_elem!(p, arr, p.with(|__s| __s.n)).write({ p.with(|__s| __s.total) });
    {
        field!(p, n).with_mut(|__v| *__v = *__v + 1_())
    };
    array_elem!(p, arr, p.with(|__s| __s.n)).write(q.with(|__s| __s.in_.x));
    unimplemented!();
    unimplemented!();
    let s: Value<Node> = <Value<Node>>::default();
    (*s.borrow_mut()).x = 1_();
    (*s.borrow_mut()).self_ = { (s.as_pointer()) };
    field!({ (*s.borrow()).self_.clone() }, x).write({ ({ (*s.borrow()).x } + 1_()) });
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
