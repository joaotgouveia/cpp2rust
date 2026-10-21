extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_int_ {
    #[offset(0)]
    #[byte_size(24)]
    vec_: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_char_ {
    #[offset(0)]
    #[byte_size(24)]
    vec_: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(24)]
pub struct MyContainer_float_ {
    #[offset(0)]
    #[byte_size(24)]
    vec_: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(2)]
pub struct Boxed_short_ {
    #[offset(0)]
    pub value: (),
}
impl Boxed_short_ {
    pub fn twice(mut v: ()) -> () {
        return (v + v);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Boxed_int_ {
    #[offset(0)]
    pub value: (),
}
impl Boxed_int_ {
    pub fn twice(mut v: ()) -> () {
        return (v + v);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Boxed_long_ {
    #[offset(0)]
    pub value: (),
}
impl Boxed_long_ {
    pub fn twice(mut v: ()) -> () {
        return (v + v);
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_int__Inner_int_ {
    #[offset(0)]
    pub t: (),
    #[offset(4)]
    pub u: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Outer_int_ {
    #[offset(0)]
    pub v: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Outer_long__Inner_int_ {
    #[offset(0)]
    pub t: (),
    #[offset(8)]
    pub u: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct Outer_long__Inner_char_ {
    #[offset(0)]
    pub t: (),
    #[offset(8)]
    pub u: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer_long_ {
    #[offset(0)]
    pub v: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Elem {
    #[offset(0)]
    pub i: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut ev: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let oi: Value<Outer_int_> = Rc::new(RefCell::new(Outer_int_ { v: 3_() }));
    unimplemented!();
    let ol: Value<Outer_long_> = Rc::new(RefCell::new(Outer_long_ { v: 5_() }));
    let ic: Value<Outer_long__Inner_char_> = Rc::new(RefCell::new(Outer_long__Inner_char_ {
        t: 6_(),
        u: ('a' as ()),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let bi: Value<Boxed_int_> = Rc::new(RefCell::new(Boxed_int_ { value: 4_() }));
    unimplemented!();
    unimplemented!();
    let bl: Value<Boxed_long_> = Rc::new(RefCell::new(Boxed_long_ { value: 7_() }));
    unimplemented!();
    let imc: Value<MyContainer_int_> = Rc::new(RefCell::new(<MyContainer_int_>::default()));
    unimplemented!();
    ({
        let _item: Value<()> = Rc::new(RefCell::new(1_()));
        MyContainer_int_Impl::push_back(&imc.as_pointer(), _item.as_pointer())
    });
    unimplemented!();
    ({ MyContainer_int_Impl::pop_back(&imc.as_pointer()) });
    unimplemented!();
    let cmc: Value<MyContainer_char_> = Rc::new(RefCell::new(<MyContainer_char_>::default()));
    unimplemented!();
    ({
        let _item: Value<()> = Rc::new(RefCell::new(('a' as ())));
        MyContainer_char_Impl::push_back(&cmc.as_pointer(), _item.as_pointer())
    });
    unimplemented!();
    ({ MyContainer_char_Impl::pop_back(&cmc.as_pointer()) });
    unimplemented!();
    let fmc: Value<MyContainer_float_> = Rc::new(RefCell::new(<MyContainer_float_>::default()));
    unimplemented!();
    ({
        let _item: Value<()> = Rc::new(RefCell::new(1_f64));
        MyContainer_float_Impl::push_back(&fmc.as_pointer(), _item.as_pointer())
    });
    unimplemented!();
    ({ MyContainer_float_Impl::pop_back(&fmc.as_pointer()) });
    unimplemented!();
    return unimplemented!();
}
pub trait Boxed_int_Impl {
    fn plus(&self, other: ()) -> ();
}
impl Boxed_int_Impl for Ptr<Boxed_int_> {
    fn plus(&self, mut other: ()) -> () {
        return ((*self).with(|__s| __s.value) + other);
    }
}
pub trait Boxed_long_Impl {
    fn plus(&self, other: ()) -> ();
}
impl Boxed_long_Impl for Ptr<Boxed_long_> {
    fn plus(&self, mut other: ()) -> () {
        return ((*self).with(|__s| __s.value) + other);
    }
}
pub trait Boxed_short_Impl {
    fn plus(&self, other: ()) -> ();
}
impl Boxed_short_Impl for Ptr<Boxed_short_> {
    fn plus(&self, mut other: ()) -> () {
        return ((*self).with(|__s| __s.value) + other);
    }
}
pub trait MyContainer_char_Impl {
    fn empty(&self) -> ();
    fn size(&self) -> ();
    fn back_3(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<()>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<()>);
}
impl MyContainer_char_Impl for Ptr<MyContainer_char_> {
    fn empty(&self) -> () {
        return unimplemented!();
    }
    fn size(&self) -> () {
        return unimplemented!();
    }
    fn back_4(&self) -> Ptr<()> {
        return unimplemented!();
    }
    fn pop_back(&self) {
        unimplemented!();
        return;
    }
    fn push_back(&self, item: Ptr<()>) {
        unimplemented!();
    }
}
pub trait MyContainer_float_Impl {
    fn empty(&self) -> ();
    fn size(&self) -> ();
    fn back_3(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<()>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<()>);
}
impl MyContainer_float_Impl for Ptr<MyContainer_float_> {
    fn empty(&self) -> () {
        return unimplemented!();
    }
    fn size(&self) -> () {
        return unimplemented!();
    }
    fn back_4(&self) -> Ptr<()> {
        return unimplemented!();
    }
    fn pop_back(&self) {
        unimplemented!();
        return;
    }
    fn push_back(&self, item: Ptr<()>) {
        unimplemented!();
    }
}
pub trait MyContainer_int_Impl {
    fn empty(&self) -> ();
    fn size(&self) -> ();
    fn back_3(&self) -> Ptr<()> {
        unimplemented!()
    }
    fn back_4(&self) -> Ptr<()>;
    fn pop_back(&self);
    fn push_back(&self, item: Ptr<()>);
}
impl MyContainer_int_Impl for Ptr<MyContainer_int_> {
    fn empty(&self) -> () {
        return unimplemented!();
    }
    fn size(&self) -> () {
        return unimplemented!();
    }
    fn back_4(&self) -> Ptr<()> {
        return unimplemented!();
    }
    fn pop_back(&self) {
        unimplemented!();
        return;
    }
    fn push_back(&self, item: Ptr<()>) {
        unimplemented!();
    }
}
pub trait Outer_int_Impl {
    fn with(&self, n: ()) -> Outer_int__Inner_int_;
}
impl Outer_int_Impl for Ptr<Outer_int_> {
    fn with(&self, mut n: ()) -> Outer_int__Inner_int_ {
        return Outer_int__Inner_int_ {
            t: (*self).with(|__s| __s.v),
            u: n,
        };
    }
}
pub trait Outer_int__Inner_int_Impl {
    fn sum(&self) -> ();
}
impl Outer_int__Inner_int_Impl for Ptr<Outer_int__Inner_int_> {
    fn sum(&self) -> () {
        return (((*self).with(|__s| __s.t) as ()) + ((*self).with(|__s| __s.u) as ()));
    }
}
pub trait Outer_long_Impl {
    fn with(&self, n: ()) -> Outer_long__Inner_int_;
}
impl Outer_long_Impl for Ptr<Outer_long_> {
    fn with(&self, mut n: ()) -> Outer_long__Inner_int_ {
        return Outer_long__Inner_int_ {
            t: (*self).with(|__s| __s.v),
            u: n,
        };
    }
}
pub trait Outer_long__Inner_char_Impl {
    fn sum(&self) -> ();
}
impl Outer_long__Inner_char_Impl for Ptr<Outer_long__Inner_char_> {
    fn sum(&self) -> () {
        return ((*self).with(|__s| __s.t) + (*self).with(|__s| __s.u));
    }
}
pub trait Outer_long__Inner_int_Impl {
    fn sum(&self) -> ();
}
impl Outer_long__Inner_int_Impl for Ptr<Outer_long__Inner_int_> {
    fn sum(&self) -> () {
        return ((*self).with(|__s| __s.t) + ((*self).with(|__s| __s.u) as ()));
    }
}
pub fn __cpp2rust_init_globals() {}
