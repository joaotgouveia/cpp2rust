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
pub fn operator_comma_0(a: Ptr<S>, b: Ptr<S>) -> S {
    return S {
        v: ({ (a.with(|__s| __s.v) * 10_()) } + { b.with(|__s| __s.v) }),
    };
}
pub fn operator_literal__k_1(mut v: ()) -> () {
    return (v).wrapping_mul(1000_());
}
pub fn operator_literal__half_2(mut v: ()) -> () {
    return (v / 2_());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 3_() }));
    let t: Value<S> = Rc::new(RefCell::new(S { v: 4_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
