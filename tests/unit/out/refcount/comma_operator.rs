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
pub fn twice_0(v: Ptr<()>) -> () {
    return ((v.read()) * 2_());
}
pub fn sum_1(p: Ptr<Pair>) -> () {
    return ({ p.with(|__s| __s.first) } + { p.with(|__s| __s.second) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: () = 1_();
    let mut y: () = {
        x = 2_();
        (x + 1_())
    };
    unimplemented!();
    unimplemented!();
    let mut z: () = {
        {
            1_();
            2_()
        };
        3_()
    };
    unimplemented!();
    let counter: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut w: () = {
        {
            (*counter.borrow_mut()).postfix_inc();
            (*counter.borrow_mut()).postfix_inc()
        };
        (*counter.borrow())
    };
    unimplemented!();
    unimplemented!();
    let mut a: () = unimplemented!();
    let mut b: () = unimplemented!();
    if {
        {
            a = 1_();
            b = 2_()
        };
        ((a + b) > unimplemented!())
    } {
        unimplemented!();
        unimplemented!();
    }
    let v1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let mut v2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let p1: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: 1_(),
        second: 2_(),
    }));
    let mut p2: Pair = {
        b = 6_();
        (*p1.borrow()).clone()
    };
    p2.first = 10_();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
