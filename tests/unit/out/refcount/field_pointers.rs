extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(12)]
pub struct Inner {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    #[byte_size(8)]
    pub name: Box<[()]>,
}
impl Default for Inner {
    fn default() -> Self {
        {
            Inner {
                a: unimplemented!(),
                name: (0..8).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Header {
    #[offset(0)]
    pub tag: (),
    #[offset(4)]
    pub size: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(104)]
pub struct Outer {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    #[byte_size(12)]
    pub inner: Inner,
    #[offset(16)]
    #[byte_size(36)]
    pub items: Box<[Inner]>,
    #[offset(56)]
    #[byte_size(24)]
    pub v: (),
    #[offset(80)]
    #[byte_size(8)]
    pub cursor: Ptr<()>,
    #[offset(88)]
    #[byte_size(16)]
    pub buf: Box<[()]>,
}
impl Outer {
    pub fn move_from(_a0: Ptr<Outer>) -> Self {
        Self {
            x: { (*_a0.upgrade().deref()).x },
            inner: { (*_a0.upgrade().deref()).inner.clone() },
            items: Box::new(std::array::from_fn::<_, 3, _>(|__i: usize| {
                ((*_a0.upgrade().deref()).items[(__i) as usize]).clone()
            })),
            v: unimplemented!(),
            cursor: { (*_a0.upgrade().deref()).cursor.clone() },
            buf: Box::new(std::array::from_fn::<_, 4, _>(|__i: usize| {
                (*_a0.upgrade().deref()).buf[(__i) as usize]
            })),
        }
    }
}
impl Default for Outer {
    fn default() -> Self {
        {
            Outer {
                x: unimplemented!(),
                inner: <Inner>::default(),
                items: (0..3).map(|_| <Inner>::default()).collect::<Box<[Inner]>>(),
                v: unimplemented!(),
                cursor: Ptr::<()>::null(),
                buf: (0..4).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
pub fn set_0(mut p: Ptr<()>, mut value: ()) {
    p.write({ value });
}
pub fn bump_1(mut o: Ptr<Outer>) -> () {
    field!(o, x).with_mut(|__v| __v.postfix_inc());
    return 1_();
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let o: Value<Outer> = Rc::new(RefCell::new(<Outer>::default()));
    (*o.borrow_mut()).x = 1_();
    let mut px: Ptr<()> = (field_ptr!(o.as_pointer(), x));
    px.write(2_());
    unimplemented!();
    let mut pa: Ptr<()> = (field_ptr!(field_ptr!(o.as_pointer(), inner), a));
    ({
        let _p: Ptr<()> = (pa).clone();
        set_0(_p, 3_())
    });
    unimplemented!();
    let mut pi: Ptr<()> = (field_ptr!(
        (array_field_ptr!(o.as_pointer(), items) as Ptr<Inner>).offset((1_()) as isize),
        a
    ));
    pi.write(4_());
    unimplemented!();
    unimplemented!();
    let mut name: Ptr<()> = (array_field_ptr!(field_ptr!(o.as_pointer(), inner), name) as Ptr<()>);
    let mut i: () = 0_();
    'loop_: while (i < 3_()) {
        elem!(name, i).write({ (('a' as ()) + i) });
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    (*o.borrow_mut()).cursor =
        { ((array_field_ptr!(o.as_pointer(), buf) as Ptr<()>).offset((1_()) as isize)) };
    { (*o.borrow()).cursor.clone() }.write(5_());
    {
        { (*o.borrow_mut()).cursor.postfix_inc() }.with_mut(|__v| *__v = *__v + 1_())
    };
    unimplemented!();
    (*o.borrow_mut()).x = 0_();
    let mut first: () = ({ OuterImpl::next(&o.as_pointer()) });
    unimplemented!();
    let __rhs = ({ (*o.borrow()).inner.a } + ({ bump_1((o.as_pointer())) }));
    (*o.borrow_mut()).x = __rhs;
    unimplemented!();
    ({
        let _k: () = { (*o.borrow()).inner.a };
        OuterImpl::push(&o.as_pointer(), _k)
    });
    unimplemented!();
    unimplemented!();
    let y: Value<()> = Rc::new(RefCell::new(0_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let bytes: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ])));
    let mut view: Ptr<Header> = (bytes.as_pointer() as Ptr<()>).reinterpret_cast::<Header>();
    field!(view, tag).write(16909060_());
    unimplemented!();
    ({ set_0((field_ptr!(view, tag)), 0_()) });
    unimplemented!();
    field!(view, size).write(1_());
    unimplemented!();
    return 0_();
}
pub trait OuterImpl {
    fn next(&self) -> ();
    fn sum(&self) -> ();
    fn push(&self, k: ());
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer>;
}
impl OuterImpl for Ptr<Outer> {
    fn next(&self) -> () {
        return (array_elem!(
            (*self),
            buf,
            field!((*self), x).with_mut(|__v| __v.postfix_inc())
        )
        .read());
    }
    fn sum(&self) -> () {
        return ((*self).with(|__s| __s.x) + (*self).with(|__s| __s.inner.a));
    }
    fn push(&self, mut k: ()) {
        unimplemented!();
    }
    fn move_assign(&self, _a0: Ptr<Outer>) -> Ptr<Outer> {
        field!((*self), x).write({ { (*_a0.upgrade().deref()).x } });
        field!((*self), inner).write({ { (*_a0.upgrade().deref()).inner.clone() } });
        unimplemented!();
        unimplemented!();
        field!((*self), cursor).write({ { (*_a0.upgrade().deref()).cursor.clone() } });
        unimplemented!();
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
