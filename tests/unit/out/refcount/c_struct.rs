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
pub struct Point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Line {
    #[offset(0)]
    #[byte_size(8)]
    pub start: Point,
    #[offset(8)]
    #[byte_size(8)]
    pub end: Point,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Node {
    #[offset(0)]
    pub value: (),
    #[offset(8)]
    #[byte_size(8)]
    pub next: Ptr<Node>,
}
pub type Color = ();
pub const Color_RED: Color = 0;
pub const Color_GREEN: Color = 1;
pub const Color_BLUE: Color = 2;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Inner {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Container {
    #[offset(0)]
    #[byte_size(8)]
    pub inner: Inner,
    #[offset(8)]
    pub color: Color,
    #[offset(12)]
    pub count: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut p: Point = Point { x: 10_(), y: 20_() };
    unimplemented!();
    unimplemented!();
    let mut q: Point = (p).clone();
    q.x = 99_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut l: Line = Line {
        start: Point { x: 1_(), y: 2_() },
        end: Point { x: 3_(), y: 4_() },
    };
    unimplemented!();
    unimplemented!();
    let a: Value<Node> = Rc::new(RefCell::new(Node {
        value: 1_(),
        next: Ptr::<Node>::null(),
    }));
    let mut b: Node = Node {
        value: 2_(),
        next: (a.as_pointer()),
    };
    unimplemented!();
    let mut c: Container = Container {
        inner: Inner { a: 5_(), b: 6_() },
        color: Color_GREEN,
        count: 42_(),
    };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut c2: Container = <Container>::default();
    c2.color = Color_BLUE;
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
