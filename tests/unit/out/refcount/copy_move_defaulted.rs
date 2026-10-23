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
pub struct Inner {
    #[offset(0)]
    pub x: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Destructor)]
#[byte_size(16)]
pub struct Explicit {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Box<[()]>,
}
impl Explicit {
    pub fn new(mut v: ()) -> Self {
        Self {
            v: v,
            inner: Inner { x: (v * 10_()) },
            arr: Box::new([v, (v + 1_())]),
        }
    }
}
impl Default for Explicit {
    fn default() -> Self {
        {
            Explicit {
                v: unimplemented!(),
                inner: <Inner>::default(),
                arr: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Implicit {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(8)]
    #[byte_size(8)]
    pub arr: Box<[()]>,
}
impl Default for Implicit {
    fn default() -> Self {
        {
            Implicit {
                v: unimplemented!(),
                inner: <Inner>::default(),
                arr: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(4)]
pub struct DefaultCopyUserMove {
    #[offset(0)]
    pub v: (),
}
impl DefaultCopyUserMove {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<DefaultCopyUserMove>) -> Self {
        let __this: DefaultCopyUserMove = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0_());
        __this
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, MoveCtor, CopyAssign, MoveAssign, Default)]
#[byte_size(4)]
pub struct UserCopyDefaultMove {
    #[offset(0)]
    pub v: (),
}
impl UserCopyDefaultMove {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<UserCopyDefaultMove>) -> Self {
        Self {
            v: (o.with(|__s| __s.v) + 100_()),
        }
    }
    pub fn move_from(_a0: Ptr<UserCopyDefaultMove>) -> Self {
        Self {
            v: { (*_a0.upgrade().deref()).v },
        }
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        let __src: Value<UserCopyDefaultMove> =
            Rc::new(RefCell::new(UserCopyDefaultMove { v: self.v.clone() }));
        UserCopyDefaultMove::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(64)]
pub struct Buffer {
    #[offset(0)]
    #[byte_size(24)]
    pub data: (),
    #[offset(24)]
    #[byte_size(24)]
    pub rows: (),
    #[offset(48)]
    pub n: (),
    #[offset(52)]
    #[byte_size(8)]
    pub arr: Box<[()]>,
}
impl Buffer {
    pub fn new(mut n: ()) -> Self {
        let __this: Buffer = Self {
            data: unimplemented!(),
            rows: unimplemented!(),
            n: n,
            arr: Box::new([n, (n + 1_())]),
        };
        unimplemented!();
        __this
    }
    pub fn move_from(_a0: Ptr<Buffer>) -> Self {
        Self {
            data: unimplemented!(),
            rows: unimplemented!(),
            n: { (*_a0.upgrade().deref()).n },
            arr: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                (*_a0.upgrade().deref()).arr[(__i) as usize]
            })),
        }
    }
}
impl Default for Buffer {
    fn default() -> Self {
        {
            Buffer {
                data: unimplemented!(),
                rows: unimplemented!(),
                n: unimplemented!(),
                arr: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(48)]
pub struct Owner {
    #[offset(0)]
    #[byte_size(24)]
    pub data: (),
    #[offset(24)]
    pub n: (),
    #[offset(28)]
    #[byte_size(8)]
    pub arr: Box<[()]>,
    #[offset(40)]
    #[byte_size(8)]
    pub p: (),
}
impl Owner {
    pub fn move_from(_a0: Ptr<Owner>) -> Self {
        Self {
            data: unimplemented!(),
            n: { (*_a0.upgrade().deref()).n },
            arr: Box::new(std::array::from_fn::<_, 2, _>(|__i: usize| {
                (*_a0.upgrade().deref()).arr[(__i) as usize]
            })),
            p: unimplemented!(),
        }
    }
}
impl Default for Owner {
    fn default() -> Self {
        {
            Owner {
                data: unimplemented!(),
                n: unimplemented!(),
                arr: (0..2).map(|_| unimplemented!()).collect::<Box<[()]>>(),
                p: unimplemented!(),
            }
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Destructor, Default)]
#[byte_size(32)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: Inner,
    #[offset(4)]
    #[byte_size(16)]
    pub e: Explicit,
    #[offset(24)]
    #[byte_size(8)]
    pub p: (),
}
impl Holder {
    pub fn new(mut v: ()) -> Self {
        Self {
            inner: Inner { x: v },
            e: Explicit::new({ v }),
            p: unimplemented!(),
        }
    }
    pub fn move_from(_a0: Ptr<Holder>) -> Self {
        Self {
            inner: { (*_a0.upgrade().deref()).inner.clone() },
            e: { (*_a0.upgrade().deref()).e.clone() },
            p: unimplemented!(),
        }
    }
}
pub fn same_0(a: Ptr<Explicit>, b: Ptr<Explicit>) -> () {
    return ((({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) })
        && ({ a.with(|__s| __s.inner.x) } == { b.with(|__s| __s.inner.x) }))
        && ({ (array_elem!(a, arr, 0_()).read()) } == { (array_elem!(b, arr, 0_()).read()) }))
        && ({ (array_elem!(a, arr, 1_()).read()) } == { (array_elem!(b, arr, 1_()).read()) });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 1_() })));
    let _dtor_a = ScopedDestructor::new(&a, |__p| __p.destructor());
    let b: Value<Explicit> = Rc::new(RefCell::new((*a.borrow()).clone()));
    let _dtor_b = ScopedDestructor::new(&b, |__p| __p.destructor());
    let c: Value<Explicit> = Rc::new(RefCell::new((*a.borrow()).clone()));
    let _dtor_c = ScopedDestructor::new(&c, |__p| __p.destructor());
    let d: Value<Explicit> = Rc::new(RefCell::new((*unimplemented!().upgrade().deref()).clone()));
    let _dtor_d = ScopedDestructor::new(&d, |__p| __p.destructor());
    unimplemented!();
    let e: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 2_() })));
    let _dtor_e = ScopedDestructor::new(&e, |__p| __p.destructor());
    let f: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 3_() })));
    let _dtor_f = ScopedDestructor::new(&f, |__p| __p.destructor());
    (*e.borrow_mut()) = (*b.borrow()).clone();
    (*f.borrow_mut()) = (*unimplemented!().upgrade().deref()).clone();
    unimplemented!();
    let g: Value<Explicit> = Rc::new(RefCell::new(Explicit::new({ 4_() })));
    let _dtor_g = ScopedDestructor::new(&g, |__p| __p.destructor());
    (*g.borrow_mut()) = {
        (*e.borrow_mut()) = (*f.borrow()).clone();
        (*e.borrow()).clone()
    };
    unimplemented!();
    let mut i: Implicit = Implicit {
        v: 5_(),
        inner: Inner { x: 50_() },
        arr: Box::new([5_(), 6_()]),
    };
    let mut j: Implicit = (i).clone();
    let mut k: Implicit = (*unimplemented!().upgrade().deref()).clone();
    unimplemented!();
    unimplemented!();
    let mut l: Implicit = Implicit {
        v: 0_(),
        inner: Inner { x: 0_() },
        arr: Box::new([0_(), 0_()]),
    };
    l = (j).clone();
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut m: DefaultCopyUserMove = DefaultCopyUserMove::new({ 7_() });
    let mut m1: DefaultCopyUserMove = (m).clone();
    let mut m2: DefaultCopyUserMove = DefaultCopyUserMove::move_from({ unimplemented!() });
    unimplemented!();
    let mut m3: DefaultCopyUserMove = DefaultCopyUserMove::new({ 1_() });
    let m4: Value<DefaultCopyUserMove> = Rc::new(RefCell::new(DefaultCopyUserMove::new({ 1_() })));
    m3 = (m1).clone();
    ({ DefaultCopyUserMoveImpl::move_assign(&m4.as_pointer(), unimplemented!()) });
    unimplemented!();
    let u: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 8_() })));
    let mut u1: UserCopyDefaultMove = UserCopyDefaultMove::copy_from({ u.as_pointer() });
    let u2: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::move_from({
        unimplemented!()
    })));
    unimplemented!();
    let u3: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 1_() })));
    let u4: Value<UserCopyDefaultMove> = Rc::new(RefCell::new(UserCopyDefaultMove::new({ 1_() })));
    ({ UserCopyDefaultMoveImpl::copy_assign(&u3.as_pointer(), u2.as_pointer()) });
    ({ UserCopyDefaultMoveImpl::move_assign(&u4.as_pointer(), unimplemented!()) });
    unimplemented!();
    let mut p: Buffer = Buffer::new({ 3_() });
    let mut q: Buffer = Buffer::move_from({ unimplemented!() });
    unimplemented!();
    let r: Value<Buffer> = Rc::new(RefCell::new(Buffer::new({ 1_() })));
    ({ BufferImpl::move_assign(&r.as_pointer(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
    let mut bufs: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut o1: Owner = <Owner>::default();
    unimplemented!();
    o1.n = 5_();
    o1.arr[(0_()) as usize] = 5_();
    o1.arr[(1_()) as usize] = 6_();
    unimplemented!();
    let mut o2: Owner = Owner::move_from({ unimplemented!() });
    unimplemented!();
    unimplemented!();
    let o3: Value<Owner> = Rc::new(RefCell::new(<Owner>::default()));
    ({ OwnerImpl::move_assign(&o3.as_pointer(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
    let h1: Value<Holder> = Rc::new(RefCell::new(Holder::new({ 4_() })));
    let _dtor_h1 = ScopedDestructor::new(&h1, |__p| __p.destructor());
    unimplemented!();
    let h2: Value<Holder> = Rc::new(RefCell::new(Holder::move_from({ unimplemented!() })));
    let _dtor_h2 = ScopedDestructor::new(&h2, |__p| __p.destructor());
    unimplemented!();
    let h3: Value<Holder> = Rc::new(RefCell::new(Holder::new({ 1_() })));
    let _dtor_h3 = ScopedDestructor::new(&h3, |__p| __p.destructor());
    ({ HolderImpl::move_assign(&h3.as_pointer(), unimplemented!()) });
    unimplemented!();
    return 0_();
}
pub trait BufferImpl {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer>;
}
impl BufferImpl for Ptr<Buffer> {
    fn move_assign(&self, _a0: Ptr<Buffer>) -> Ptr<Buffer> {
        unimplemented!();
        unimplemented!();
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        unimplemented!();
        return (*self).clone();
    }
}
pub trait DefaultCopyUserMoveImpl {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove>;
}
impl DefaultCopyUserMoveImpl for Ptr<DefaultCopyUserMove> {
    fn move_assign(&self, o: Ptr<DefaultCopyUserMove>) -> Ptr<DefaultCopyUserMove> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0_());
        return (*self).clone();
    }
}
pub trait ExplicitImpl {
    fn destructor(&self);
}
impl ExplicitImpl for Ptr<Explicit> {
    fn destructor(&self) {}
}
pub trait HolderImpl {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
    fn destructor(&self);
}
impl HolderImpl for Ptr<Holder> {
    fn move_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        field!((*self), inner).write({ { (*_a0.upgrade().deref()).inner.clone() } });
        field!((*self), e).write({ { (*_a0.upgrade().deref()).e.clone() } });
        unimplemented!();
        return (*self).clone();
    }
    fn destructor(&self) {
        field_ptr!(self, e).destructor();
    }
}
pub trait OwnerImpl {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner>;
}
impl OwnerImpl for Ptr<Owner> {
    fn move_assign(&self, _a0: Ptr<Owner>) -> Ptr<Owner> {
        unimplemented!();
        field!((*self), n).write({ { (*_a0.upgrade().deref()).n } });
        unimplemented!();
        unimplemented!();
        return (*self).clone();
    }
}
pub trait UserCopyDefaultMoveImpl {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove>;
}
impl UserCopyDefaultMoveImpl for Ptr<UserCopyDefaultMove> {
    fn copy_assign(&self, o: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        field!((*self), v).write({ (o.with(|__s| __s.v) + 100_()) });
        return (*self).clone();
    }
    fn move_assign(&self, _a0: Ptr<UserCopyDefaultMove>) -> Ptr<UserCopyDefaultMove> {
        field!((*self), v).write({ { (*_a0.upgrade().deref()).v } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
