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
pub fn operator_eq_0(mut a: (), b: Ptr<S>) -> () {
    return ({ a } == { b.with(|__s| __s.v) });
}
pub fn operator_ne_1(mut a: (), b: Ptr<S>) -> () {
    return ({ a } != { b.with(|__s| __s.v) });
}
pub fn operator_lt_2(mut a: (), b: Ptr<S>) -> () {
    return ({ a } < { b.with(|__s| __s.v) });
}
pub fn operator_gt_3(mut a: (), b: Ptr<S>) -> () {
    return ({ a } > { b.with(|__s| __s.v) });
}
pub fn operator_le_4(mut a: (), b: Ptr<S>) -> () {
    return ({ a } <= { b.with(|__s| __s.v) });
}
pub fn operator_ge_5(mut a: Ptr<()>, b: Ptr<S>) -> () {
    return ({ ((a.read()) - ('0' as ())) } >= { b.with(|__s| __s.v) });
}
pub fn operator_lt_6(a: Ptr<S>, mut b: ()) -> () {
    return ({ (a.with(|__s| __s.v) + 1_()) } < { b });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 5_() }));
    let cs: Ptr<S> = s.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait SImpl {
    fn operator_eq(&self, o: ()) -> ();
    fn operator_ne(&self, o: ()) -> ();
    fn operator_lt(&self, o: ()) -> ();
    fn operator_gt(&self, o: ()) -> ();
    fn operator_le(&self, o: ()) -> ();
    fn operator_ge(&self, o: Ptr<()>) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, mut o: ()) -> () {
        return ((*self).with(|__s| __s.v) == o);
    }
    fn operator_ne(&self, mut o: ()) -> () {
        return ((*self).with(|__s| __s.v) != o);
    }
    fn operator_lt(&self, mut o: ()) -> () {
        return ((*self).with(|__s| __s.v) < o);
    }
    fn operator_gt(&self, mut o: ()) -> () {
        return ((*self).with(|__s| __s.v) > o);
    }
    fn operator_le(&self, mut o: ()) -> () {
        return ((*self).with(|__s| __s.v) <= o);
    }
    fn operator_ge(&self, mut o: Ptr<()>) -> () {
        return ({ (*self).with(|__s| __s.v) } >= { ((o.read()) - ('0' as ())) });
    }
}
pub fn __cpp2rust_init_globals() {}
