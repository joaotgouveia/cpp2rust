extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static total_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub v: (),
}
impl S {
    pub fn new(mut init: ()) -> Self {
        let __this: Value<S> = Rc::new(RefCell::new(Self { v: init }));
        let this: Ptr<S> = __this.as_pointer();
        ({ SImpl::mut_method(&this) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(&this) }));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Point {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
impl Point {
    pub fn new_1(mut x: (), mut y: ()) -> Self {
        Self { x: x, y: y }
    }
    pub fn new_2(mut v: ()) -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_1({ v }, { (v + 1_()) })));
        let this: Ptr<Point> = __this.as_pointer();
        {
            field!(this, y).with_mut(|__v| *__v = *__v * 10_())
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
    pub fn new_3() -> Self {
        let __this: Value<Point> = Rc::new(RefCell::new(Point::new_2({ 4_() })));
        let this: Ptr<Point> = __this.as_pointer();
        {
            field!(this, x).with_mut(|__v| *__v = *__v + 100_())
        };
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for Point {
    fn default() -> Self {
        { Point::new_3() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    {
        let s: Value<S> = Rc::new(RefCell::new(S::new({ 3_() })));
        let _dtor_s = ScopedDestructor::new(&s, |__p| __p.destructor());
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    let mut p: Point = Point::new_3();
    unimplemented!();
    unimplemented!();
    let mut q: Point = Point::new_2({ 7_() });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait SImpl {
    fn const_method(&self) -> ();
    fn mut_method(&self);
    fn destructor(&self);
}
impl SImpl for Ptr<S> {
    fn const_method(&self) -> () {
        return ((*self).with(|__s| __s.v) * 2_());
    }
    fn mut_method(&self) {
        {
            field!((*self), v).with_mut(|__v| *__v = *__v + 1_())
        };
    }
    fn destructor(&self) {
        ({ SImpl::mut_method(self) });
        total_0.with(|rc| *rc.borrow_mut() += ({ SImpl::const_method(self) }));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = total_0.with(|_| ());
}
