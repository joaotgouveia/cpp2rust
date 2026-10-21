extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S {
    #[offset(0)]
    pub a_: (),
    #[offset(8)]
    #[byte_size(8)]
    pub self__: Ptr<S>,
}
impl S {
    pub fn new_1(mut a: ()) -> Self {
        Self {
            a_: a,
            self__: Ptr::<S>::null(),
        }
    }
    pub fn new_2(mut a: (), mut other: Ptr<S>) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: a,
            self__: Ptr::<S>::null(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        if (this == other) {
            field!(this, self__).write(Ptr::<S>::null());
        } else {
            field!(this, self__).write((other).clone());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3(mut a: (), mut other: Ptr<S>) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self {
            a_: a,
            self__: Ptr::<S>::null(),
        }));
        let this: Ptr<S> = __this.as_pointer();
        if (this == other) {
            field!(this, self__).write(Ptr::<S>::null());
        }
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn bump_0(mut p: Ptr<S>) {
    field!(p, a_).with_mut(|__v| __v.postfix_inc());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct D {
    #[offset(0)]
    pub a_: (),
}
impl D {
    pub fn new(mut a: ()) -> Self {
        let __this: Value<D> = Rc::new(RefCell::new(Self { a_: a }));
        let this: Ptr<D> = __this.as_pointer();
        {
            field!(this, a_).with_mut(|__v| *__v = *__v * 2_())
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S::new_1({ 1_() })));
    let ref_: Ptr<S> = ({ SImpl::returns_this_reference(&s.as_pointer()) });
    field!(ref_, a_).with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    let mut ptr: Ptr<S> = ({ SImpl::returns_this_pointer(&s.as_pointer()) });
    field!(ptr, a_).with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    ({ SImpl::inc(&({ SImpl::inc(&({ SImpl::inc(&s.as_pointer()) })) })) });
    unimplemented!();
    ({ SImpl::set_from_this(&s.as_pointer()) });
    unimplemented!();
    unimplemented!();
    ({ SImpl::link(&s.as_pointer()) });
    unimplemented!();
    field!({ (*s.borrow()).self__.clone() }, a_).with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    ({ SImpl::bump_me(&s.as_pointer()) });
    unimplemented!();
    let mut d: D = D::new({ 3_() });
    unimplemented!();
    let cr: Ptr<S> = ({ SImpl::cref(&s.as_pointer()) });
    unimplemented!();
    let t: Value<S> = Rc::new(RefCell::new(S::new_1({ unimplemented!() })));
    unimplemented!();
    unimplemented!();
    let mut p: Ptr<S> = Ptr::alloc(S::new_1({ 1_() }));
    let mut q: Ptr<S> = ({ SImpl::returns_this_pointer(&p) });
    field!(q, a_).with_mut(|__v| __v.postfix_inc());
    unimplemented!();
    p.delete();
    let mut h: Ptr<S> = Ptr::alloc(S::new_1({ 5_() }));
    ({ SImpl::destroy(&h) });
    ({ SImpl::reset(&s.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let other: Value<S> = Rc::new(RefCell::new(S::new_1({ 22_() })));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut u: S = S::new_2({ 1_() }, { (s.as_pointer()) });
    unimplemented!();
    let s_const: Value<S> = Rc::new(RefCell::new(S::new_1({ 100_() })));
    let mut u1: S = S::new_3({ 1_() }, { (s_const.as_pointer()) });
    unimplemented!();
    return unimplemented!();
}
pub trait SImpl {
    fn returns_this_reference(&self) -> Ptr<S>;
    fn returns_this_pointer(&self) -> Ptr<S>;
    fn inc(&self) -> Ptr<S>;
    fn set_from_this(&self);
    fn get(&self) -> ();
    fn twice(&self) -> ();
    fn link(&self);
    fn bump_me(&self);
    fn cref(&self) -> Ptr<S>;
    fn is(&self, o: Ptr<S>) -> ();
    fn destroy(&self);
    fn reset(&self);
    fn copy_if_different_const(&self, other: Ptr<S>) -> ();
    fn copy_if_different(&self, other: Ptr<S>) -> ();
}
impl SImpl for Ptr<S> {
    fn returns_this_reference(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn returns_this_pointer(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn inc(&self) -> Ptr<S> {
        field!((*self), a_).with_mut(|__v| __v.postfix_inc());
        return (*self).clone();
    }
    fn set_from_this(&self) {
        field!((*self), a_).write({ ((*self).with(|__s| __s.a_) + 1_()) });
    }
    fn get(&self) -> () {
        return (*self).with(|__s| __s.a_);
    }
    fn twice(&self) -> () {
        return (({ SImpl::get(self) }) * 2_());
    }
    fn link(&self) {
        field!((*self), self__).write((*self).clone());
    }
    fn bump_me(&self) {
        ({ bump_0((*self).clone()) });
    }
    fn cref(&self) -> Ptr<S> {
        return (*self).clone();
    }
    fn is(&self, mut o: Ptr<S>) -> () {
        return (o == (*self));
    }
    fn destroy(&self) {
        (*self).delete();
    }
    fn reset(&self) {
        (*self).write(S::new_1({ unimplemented!() }));
    }
    fn copy_if_different_const(&self, mut other: Ptr<S>) -> () {
        if ((*self) == other) {
            return false;
        }
        field!((*self), a_).write({ other.with(|__s| __s.a_) });
        field!((*self), self__).write({ other.with(|__s| __s.self__.clone()) });
        return true;
    }
    fn copy_if_different(&self, mut other: Ptr<S>) -> () {
        if ((*self) == other) {
            return false;
        }
        field!((*self), a_).write({ other.with(|__s| __s.a_) });
        field!((*self), self__).write({ other.with(|__s| __s.self__.clone()) });
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
