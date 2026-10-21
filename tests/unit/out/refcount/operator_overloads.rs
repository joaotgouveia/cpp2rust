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
pub fn operator_div_0(a: Ptr<S>, b: Ptr<S>) -> () {
    return ({ a.with(|__s| __s.v) } / { b.with(|__s| __s.v) });
}
pub fn operator_div_1(mut a: S, mut b: ()) -> () {
    return ((a.v / b) + 1_());
}
pub fn operator_rem_2(mut a: S, mut b: S) -> () {
    return (a.v % b.v);
}
pub fn operator_rem_3(a: Ptr<S>, mut b: ()) -> () {
    return (({ a.with(|__s| __s.v) } % { b }) + 1_());
}
pub fn operator_eq_4(mut a: (), mut b: S) -> () {
    return if (a == b.v) { 4_() } else { unimplemented!() };
}
pub fn operator_eq_5(mut a: (), b: Ptr<S>) -> () {
    return if ({ a } == { b.with(|__s| __s.v) }) {
        5_()
    } else {
        unimplemented!()
    };
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S { v: 6_() }));
    let t: Value<S> = Rc::new(RefCell::new(S { v: 4_() }));
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
    fn operator_eq_1(&self, o: ()) -> ();
    fn operator_eq_2(&self, o: ()) -> ();
    fn operator_eq_3(&self, o: ()) -> ();
    fn operator_add(&self, o: Ptr<S>) -> ();
    fn operator_sub(&self, o: S) -> ();
    fn operator_mul_6(&self, o: Ptr<S>) -> ();
    fn operator_mul_7(&self, o: ()) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_eq_1(&self, mut o: ()) -> () {
        return if ((*self).with(|__s| __s.v) == o) {
            1_()
        } else {
            unimplemented!()
        };
    }
    fn operator_eq_2(&self, mut o: ()) -> () {
        return if ((*self).with(|__s| __s.v) == o) {
            2_()
        } else {
            unimplemented!()
        };
    }
    fn operator_eq_3(&self, mut o: ()) -> () {
        return if ((*self).with(|__s| __s.v) == o) {
            3_()
        } else {
            unimplemented!()
        };
    }
    fn operator_add(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } + { o.with(|__s| __s.v) });
    }
    fn operator_sub(&self, mut o: S) -> () {
        return ((*self).with(|__s| __s.v) - o.v);
    }
    fn operator_mul_6(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } * { o.with(|__s| __s.v) });
    }
    fn operator_mul_7(&self, mut o: ()) -> () {
        return (((*self).with(|__s| __s.v) * o) + 1_());
    }
}
pub fn __cpp2rust_init_globals() {}
