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
impl CppLess for S {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_eq_1(a: Ptr<S>, b: Ptr<S>) -> () {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
pub fn operator_ne_2(a: Ptr<S>, b: Ptr<S>) -> () {
    return ({ a.with(|__s| __s.v) } != { b.with(|__s| __s.v) });
}
pub fn operator_lt_0(a: Ptr<S>, b: Ptr<S>) -> () {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_gt_3(a: Ptr<S>, b: Ptr<S>) -> () {
    return ({ a.with(|__s| __s.v) } > { b.with(|__s| __s.v) });
}
pub fn operator_le_4(a: Ptr<S>, b: Ptr<S>) -> () {
    return ({ a.with(|__s| __s.v) } <= { b.with(|__s| __s.v) });
}
pub fn operator_ge_5(a: Ptr<S>, b: Ptr<S>) -> () {
    return ({ a.with(|__s| __s.v) } >= { b.with(|__s| __s.v) });
}
pub fn operator_lt_6(a: Ptr<S>, mut b: ()) -> () {
    return ({ a.with(|__s| __s.v) } < { b });
}
pub fn operator_lt_7(mut a: (), b: Ptr<S>) -> () {
    return ({ a } < { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct V {
    #[offset(0)]
    pub v: (),
}
impl CppLess for V {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_8(__lhs.read(), __rhs.read())
    }
}
impl CppEq for V {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_9(__lhs.read(), __rhs.read())
    }
}
pub fn operator_eq_9(mut a: V, mut b: V) -> () {
    return (a.v == b.v);
}
pub fn operator_ne_10(mut a: V, mut b: V) -> () {
    return (a.v != b.v);
}
pub fn operator_lt_8(mut a: V, mut b: V) -> () {
    return (a.v < b.v);
}
pub fn operator_gt_11(mut a: V, mut b: V) -> () {
    return (a.v > b.v);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut x: V = V { v: 1_() };
    let mut y: V = V { v: 2_() };
    let mut z: V = V { v: 1_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let a: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2_() }));
    let c: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
    unimplemented!();
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
pub fn __cpp2rust_init_globals() {}
