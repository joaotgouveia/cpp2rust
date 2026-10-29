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
pub struct S {
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
    let s1: Value<S> = Rc::new(RefCell::new(S { a: 1_(), b: 2_() }));
    let s2: Value<S> = Rc::new(RefCell::new(S { a: 1_(), b: 2_() }));
    let s3: Value<S> = Rc::new(RefCell::new(S { a: 1_(), b: 3_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn operator_eq_0(x: Ptr<S>, y: Ptr<S>) -> () {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) })
        && ({ x.with(|__s| __s.b) } == { y.with(|__s| __s.b) });
}
pub fn operator_ne_1(x: Ptr<S>, y: Ptr<S>) -> () {
    return !({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_eq_0(_x, _y)
    });
}
pub fn operator_lt_2(x: Ptr<S>, y: Ptr<S>) -> () {
    return ({ x.with(|__s| __s.a) } < { y.with(|__s| __s.a) })
        || (({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) })
            && ({ x.with(|__s| __s.b) } < { y.with(|__s| __s.b) }));
}
pub fn compare_3(x: Ptr<S>, y: Ptr<S>) -> () {
    if ({
        let _x: Ptr<S> = (x).clone();
        let _y: Ptr<S> = (y).clone();
        operator_lt_2(_x, _y)
    }) {
        return -1_();
    }
    if ({
        let _x: Ptr<S> = (y).clone();
        let _y: Ptr<S> = (x).clone();
        operator_lt_2(_x, _y)
    }) {
        return 1_();
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
