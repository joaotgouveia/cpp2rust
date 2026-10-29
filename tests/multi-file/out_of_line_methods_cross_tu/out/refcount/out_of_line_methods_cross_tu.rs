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
impl S {}
pub trait Base {
    fn apply(&mut self, x: ()) -> ();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Derived {
    #[offset(8)]
    pub factor: (),
}
impl Derived {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Pair {
    #[offset(0)]
    pub first: (),
    #[offset(4)]
    pub second: (),
}
pub fn pair_diff_0(mut p: Ptr<Pair>) -> () {
    return ({ p.with(|__s| __s.first) } - { p.with(|__s| __s.second) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S::new({ 1_() })));
    unimplemented!();
    ({ SImpl::set(&s.as_pointer(), 4_()) });
    unimplemented!();
    unimplemented!();
    let derived: Value<Derived> = Rc::new(RefCell::new(Derived::new({ 3_() })));
    let mut base: PtrDyn<dyn Base> = (derived.as_pointer()).to_dyn::<dyn Base>(|w| w);
    unimplemented!();
    let pair: Value<Pair> = Rc::new(RefCell::new(Pair {
        first: 7_(),
        second: 3_(),
    }));
    unimplemented!();
    unimplemented!();
    return 0_();
}
impl S {
    pub fn new(mut x: ()) -> Self {
        Self { v: x }
    }
}
impl Derived {
    pub fn new(mut factor: ()) -> Self {
        Self { factor: factor }
    }
}
impl S {}
impl Derived {}
pub fn pair_sum_1(mut p: Ptr<Pair>) -> () {
    return ({ p.with(|__s| __s.first) } + { p.with(|__s| __s.second) });
}
impl Base for Derived {
    fn apply(&mut self, mut x: ()) -> () {
        return ({ self.factor } * x);
    }
}
pub trait SImpl {
    fn destructor(&self) {
        unimplemented!()
    }
    fn get(&self) -> ();
    fn set(&self, mut x: ()) {
        unimplemented!()
    }
    fn add(&self, mut x: ()) -> () {
        unimplemented!()
    }
}
impl SImpl for Ptr<S> {
    fn get(&self) -> () {
        return (*self).with(|__s| __s.v);
    }
    fn destructor(&self) {}
    fn set(&self, mut x: ()) {
        field!((*self), v).write(x);
    }
    fn add(&self, mut x: ()) -> () {
        {
            let __rhs = x;
            field!((*self), v).with_mut(|__v| *__v = *__v + __rhs)
        };
        return (*self).with(|__s| __s.v);
    }
}
pub fn __cpp2rust_init_globals() {}
