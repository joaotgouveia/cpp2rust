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
pub struct Pair {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Triple {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
    #[offset(8)]
    #[byte_size(8)]
    pub p: Pair,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p: Ptr<Pair> = Ptr::alloc(Pair { x: 1_(), y: 2_() });
    let mut out: () = ({ p.with(|__s| __s.x) } + { p.with(|__s| __s.y) });
    p.delete();
    unimplemented!();
    let mut t: Triple = Triple {
        a: 1_(),
        b: unimplemented!(),
        p: <Pair>::default(),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut q: Ptr<Triple> = Ptr::alloc(Triple {
        a: 2_(),
        b: 3_(),
        p: <Pair>::default(),
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    q.delete();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
