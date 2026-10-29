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
        SImpl::operator_lt_3(__lhs, __rhs.clone())
    }
}
impl CppEq for S {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SImpl::operator_eq(__lhs, __rhs.clone())
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
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
    return 0_();
}
pub trait SImpl {
    fn operator_eq(&self, o: Ptr<S>) -> ();
    fn operator_ne(&self, o: Ptr<S>) -> ();
    fn operator_lt_3(&self, o: Ptr<S>) -> ();
    fn operator_gt(&self, o: Ptr<S>) -> ();
    fn operator_le(&self, o: Ptr<S>) -> ();
    fn operator_ge(&self, o: Ptr<S>) -> ();
    fn operator_lt_7(&self, o: ()) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
    fn operator_ne(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } != { o.with(|__s| __s.v) });
    }
    fn operator_lt_3(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
    fn operator_gt(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } > { o.with(|__s| __s.v) });
    }
    fn operator_le(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } <= { o.with(|__s| __s.v) });
    }
    fn operator_ge(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } >= { o.with(|__s| __s.v) });
    }
    fn operator_lt_7(&self, mut o: ()) -> () {
        return ((*self).with(|__s| __s.v) < o);
    }
}
pub fn __cpp2rust_init_globals() {}
