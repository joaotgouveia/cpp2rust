extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct GraphNode {
    #[offset(0)]
    pub dst: (),
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<GraphNode>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Graph {
    #[offset(0)]
    pub V: (),
    #[offset(8)]
    #[byte_size(8)]
    pub adj: Ptr<Ptr<GraphNode>>,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Partial {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<()>,
}
impl Partial {
    pub fn new_1(mut q: Ptr<()>) -> Self {
        Self { p: (q).clone() }
    }
}
impl Default for Partial {
    fn default() -> Self {
        {
            Partial {
                p: Ptr::<()>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Declared {}
impl Declared {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub i: (),
    #[offset(8)]
    #[byte_size(8)]
    pub d: Ptr<Declared>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut g: Graph = Graph {
        V: 5_(),
        adj: Ptr::<Ptr<GraphNode>>::null(),
    };
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([3_(), 1_(), 4_()])));
    let mut it: Partial = Partial::new_1({ (arr.as_pointer() as Ptr<()>) });
    if ({ (it.p).clone() } != { (arr.as_pointer() as Ptr<()>) }) {
        return 1_();
    }
    let mut def: Partial = <Partial>::default();
    if !((def.p).is_null()) {
        return 1_();
    }
    let mut s: S = S {
        i: 7_(),
        d: Ptr::<Declared>::null(),
    };
    if (s.i != 7_()) || (!((s.d).is_null())) {
        return 1_();
    }
    return 0_();
}
pub trait GraphImpl {
    fn push(&self, src: (), dst: ());
}
impl GraphImpl for Ptr<Graph> {
    fn push(&self, mut src: (), mut dst: ()) {
        let __rhs = Ptr::alloc(GraphNode {
            dst: dst,
            next: (elem!((*self).with(|__s| __s.adj.clone()), src).read()),
        });
        elem!((*self).with(|__s| __s.adj.clone()), src).write(__rhs);
        let __rhs = Ptr::alloc(GraphNode {
            dst: src,
            next: (elem!((*self).with(|__s| __s.adj.clone()), dst).read()),
        });
        elem!((*self).with(|__s| __s.adj.clone()), dst).write(__rhs);
    }
}
pub trait PartialImpl {
    fn get(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn next_4(&self) -> Ptr<Partial> {
        unimplemented!()
    }
    fn next_5(&self, mut _a0: ()) -> Partial {
        unimplemented!()
    }
}
impl PartialImpl for Ptr<Partial> {}
pub fn __cpp2rust_init_globals() {}
