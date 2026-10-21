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
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: Ptr<()> = Ptr::alloc(5_());
    let mut out: () = (x.read());
    x.delete();
    unimplemented!();
    let mut y: Ptr<()> = Ptr::alloc(Default::default());
    y.write(9_());
    unimplemented!();
    y.delete();
    let mut p: Ptr<Pair> = Ptr::alloc(<Pair>::default());
    field!(p, x).write(1_());
    field!(p, y).write(2_());
    unimplemented!();
    p.delete();
    let mut nullpointer: Ptr<()> = Ptr::<()>::null();
    nullpointer.delete();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
