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
pub struct Foo {
    #[offset(0)]
    pub x1: (),
    #[offset(4)]
    pub x2: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let arr: Value<Box<[Foo]>> = Rc::new(RefCell::new(Box::new([
        Foo { x1: 1_(), x2: 2_() },
        Foo { x1: 3_(), x2: 4_() },
    ])));
    let mut p1: Ptr<()> = (field_ptr!((arr.as_pointer() as Ptr<Foo>).offset(1_()), x1));
    let mut a: () = (p1.read());
    let mut p2: Ptr<Foo> = ((arr.as_pointer() as Ptr<Foo>).offset(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
