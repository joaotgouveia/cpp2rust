extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Static {}
impl Static {
    pub fn operator_call(mut a: (), mut b: ()) -> () {
        return (a * b);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: (),
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
    let mut i: () = ({ SImpl::to__5(&s.as_pointer()) });
    unimplemented!();
    unimplemented!();
    if ({ SImpl::to__6(&s.as_pointer()) }) {
        unimplemented!();
    } else {
        unimplemented!();
    }
    let z: Value<S> = Rc::new(RefCell::new(S { v: 0_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let st: Value<Static> = Rc::new(RefCell::new(<Static>::default()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn operator_call_1(&self) -> ();
    fn operator_call_2(&self, a: ()) -> ();
    fn operator_call_3(&self, a: (), b: ()) -> ();
    fn operator_comma(&self, o: Ptr<S>) -> S;
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_call_1(&self) -> () {
        return (*self).with(|__s| __s.v);
    }
    fn operator_call_2(&self, mut a: ()) -> () {
        return ((*self).with(|__s| __s.v) + a);
    }
    fn operator_call_3(&self, mut a: (), mut b: ()) -> () {
        return (((*self).with(|__s| __s.v) + a) + b);
    }
    fn operator_comma(&self, o: Ptr<S>) -> S {
        return S {
            v: ({ ((*self).with(|__s| __s.v) * 10_()) } + { o.with(|__s| __s.v) }),
        };
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.v);
    }
    fn to__6(&self) -> () {
        return ((*self).with(|__s| __s.v) != 0_());
    }
}
pub fn __cpp2rust_init_globals() {}
