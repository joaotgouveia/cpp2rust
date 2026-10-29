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
    pub first: (),
    #[offset(4)]
    pub second: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let __decomp_0: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: 1_(),
        second: 2_(),
    }));
    unimplemented!();
    unimplemented!();
    let mut p: Pair = Pair {
        first: 10_(),
        second: 20_(),
    };
    let __decomp_1: Value<Pair> = Rc::new(RefCell::new((p).clone()));
    (*__decomp_1.borrow_mut()).first = 11_();
    (*__decomp_1.borrow_mut()).second += 1_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
