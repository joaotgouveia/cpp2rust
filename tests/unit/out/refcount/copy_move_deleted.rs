extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(4)]
pub struct NoCopy {
    #[offset(0)]
    pub v: (),
}
impl NoCopy {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<NoCopy>) -> Self {
        let __this: NoCopy = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0_());
        __this
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign)]
#[byte_size(4)]
pub struct PrivateCopy {
    #[offset(0)]
    pub v: (),
}
impl PrivateCopy {
    pub fn new() -> Self {
        Self { v: 0_() }
    }
    pub fn move_from(o: Ptr<PrivateCopy>) -> Self {
        let __this: PrivateCopy = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0_());
        __this
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        { PrivateCopy::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct Immovable {
    #[offset(0)]
    pub v: (),
}
impl Immovable {
    pub fn new() -> Self {
        Self { v: 0_() }
    }
}
impl Default for Immovable {
    fn default() -> Self {
        { Immovable::new() }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(8)]
pub struct Container {
    #[offset(0)]
    #[byte_size(4)]
    pub inner: NoCopy,
    #[offset(4)]
    pub tag: (),
}
impl Container {
    pub fn move_from(_a0: Ptr<Container>) -> Self {
        Self {
            inner: NoCopy::move_from({ field_ptr!(_a0, inner) }),
            tag: { (*_a0.upgrade().deref()).tag },
        }
    }
}
#[derive(Record, ByteRepr, FnPtrArg, MoveCtor, MoveAssign, Default)]
#[byte_size(24)]
pub struct VecHolder {
    #[offset(0)]
    #[byte_size(24)]
    pub items: std_vector_NoCopy__std_allocator_NoCopy__,
}
impl VecHolder {
    pub fn move_from(_a0: Ptr<VecHolder>) -> Self {
        Self {
            items: field!(_a0, items).with_mut(std::mem::take),
        }
    }
}
pub fn bump_0(mut p: Ptr<NoCopy>) {
    field!(p, v).with_mut(|__v| __v.postfix_inc());
}
pub fn bump_ref_1(r: Ptr<Immovable>) {
    field!(r, v).with_mut(|__v| __v.postfix_inc());
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<NoCopy> = Rc::new(RefCell::new(NoCopy::new({ 1_() })));
    let b: Value<NoCopy> = Rc::new(RefCell::new(NoCopy::move_from({ a.as_pointer() })));
    unimplemented!();
    ({ NoCopyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    ({ bump_0((a.as_pointer())) });
    unimplemented!();
    let p: Value<PrivateCopy> = Rc::new(RefCell::new(PrivateCopy::new()));
    (*p.borrow_mut()).v = 3_();
    let q: Value<PrivateCopy> = Rc::new(RefCell::new(PrivateCopy::move_from({ p.as_pointer() })));
    unimplemented!();
    ({ PrivateCopyImpl::move_assign(&p.as_pointer(), q.as_pointer()) });
    unimplemented!();
    let im: Value<Immovable> = Rc::new(RefCell::new(Immovable::new()));
    (*im.borrow_mut()).v = 4_();
    ({ bump_ref_1(im.as_pointer()) });
    let mut pim: Ptr<Immovable> = (im.as_pointer());
    unimplemented!();
    let c: Value<Container> = Rc::new(RefCell::new(Container {
        inner: NoCopy::new({ 6_() }),
        tag: 7_(),
    }));
    let mut d: Container = Container::move_from({ c.as_pointer() });
    unimplemented!();
    let h: Value<VecHolder> = Rc::new(RefCell::new(<VecHolder>::default()));
    let h2: Value<VecHolder> = Rc::new(RefCell::new(VecHolder::move_from({ h.as_pointer() })));
    unimplemented!();
    return 0_();
}
pub trait ContainerImpl {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container>;
}
impl ContainerImpl for Ptr<Container> {
    fn move_assign(&self, _a0: Ptr<Container>) -> Ptr<Container> {
        ({
            let _o: Ptr<NoCopy> = field_ptr!(_a0, inner);
            NoCopyImpl::move_assign(&field_ptr!((*self), inner), _o)
        });
        field!((*self), tag).write({ { (*_a0.upgrade().deref()).tag } });
        return (*self).clone();
    }
}
pub trait NoCopyImpl {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy>;
}
impl NoCopyImpl for Ptr<NoCopy> {
    fn move_assign(&self, o: Ptr<NoCopy>) -> Ptr<NoCopy> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0_());
        return (*self).clone();
    }
}
pub trait PrivateCopyImpl {
    fn copy_assign(&self, _a0: Ptr<PrivateCopy>) -> Ptr<PrivateCopy> {
        unimplemented!()
    }
    fn move_assign(&self, o: Ptr<PrivateCopy>) -> Ptr<PrivateCopy>;
}
impl PrivateCopyImpl for Ptr<PrivateCopy> {
    fn move_assign(&self, o: Ptr<PrivateCopy>) -> Ptr<PrivateCopy> {
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0_());
        return (*self).clone();
    }
}
pub trait VecHolderImpl {
    fn move_assign(&self, _a0: Ptr<VecHolder>) -> Ptr<VecHolder>;
}
impl VecHolderImpl for Ptr<VecHolder> {
    fn move_assign(&self, _a0: Ptr<VecHolder>) -> Ptr<VecHolder> {
        field!((*self), items).write({ { (*_a0.upgrade().deref()).items.clone() } });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
