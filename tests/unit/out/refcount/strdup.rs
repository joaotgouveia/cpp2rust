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
pub struct record {
    #[offset(0)]
    #[byte_size(8)]
    pub name: Ptr<()>,
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut d: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<()> = Ptr::<()>::from_string_literal(b"world");
    let buf: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        ('a' as ()),
        ('b' as ()),
        ('c' as ()),
        ('\0' as ()),
    ])));
    let mut d2: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d3: Ptr<()> = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut d4: Ptr<()> = Ptr::<()>::null();
    d4 = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let rec: Value<record> = Rc::new(RefCell::new(record {
        name: Ptr::<()>::null(),
    }));
    let mut r: Ptr<record> = (rec.as_pointer());
    let __rhs = unimplemented!();
    field!(r, name).write(__rhs);
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
