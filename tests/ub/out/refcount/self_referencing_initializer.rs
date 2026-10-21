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
pub struct pair {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: () = unimplemented!();
    x = (x + 1_());
    let mut p: pair = <pair>::default();
    p = pair {
        a: 1_(),
        b: (p.a + 1_()),
    };
    return (x + p.b);
}
pub fn __cpp2rust_init_globals() {}
