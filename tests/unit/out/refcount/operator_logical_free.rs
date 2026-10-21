extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: (),
}
pub fn operator_not_0(a: Ptr<S>) -> () {
    return (a.with(|__s| __s.v) == unimplemented!());
}
pub fn operator_and_1(a: Ptr<S>, b: Ptr<S>) -> () {
    return (a.with(|__s| __s.v) != unimplemented!()) && (b.with(|__s| __s.v) != unimplemented!());
}
pub fn operator_or_2(a: Ptr<S>, b: Ptr<S>) -> () {
    return (a.with(|__s| __s.v) != unimplemented!()) || (b.with(|__s| __s.v) != unimplemented!());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let t: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
    let f: Value<S> = Rc::new(RefCell::new(S {
        v: unimplemented!(),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub fn __cpp2rust_init_globals() {}
