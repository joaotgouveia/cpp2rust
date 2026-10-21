extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static assigns_0: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(8)]
pub struct Partial {
    #[offset(0)]
    pub v: (),
    #[offset(4)]
    pub keep: (),
}
impl Partial {
    pub fn new(mut v: (), mut keep: ()) -> Self {
        Self { v: v, keep: keep }
    }
    pub fn copy_from(o: Ptr<Partial>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            keep: o.with(|__s| __s.keep),
        }
    }
}
impl Clone for Partial {
    fn clone(&self) -> Self {
        let __src: Value<Partial> = Rc::new(RefCell::new(Partial {
            v: self.v.clone(),
            keep: self.keep.clone(),
        }));
        Partial::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NonConstAssign {
    #[offset(0)]
    pub mark: (),
}
impl NonConstAssign {
    pub fn new() -> Self {
        Self { mark: 0_() }
    }
}
impl Default for NonConstAssign {
    fn default() -> Self {
        { NonConstAssign::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(4)]
pub struct RefQualified {
    #[offset(0)]
    pub mark: (),
}
impl RefQualified {
    pub fn new() -> Self {
        Self { mark: 0_() }
    }
}
impl Default for RefQualified {
    fn default() -> Self {
        { RefQualified::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(24)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Partial,
    #[offset(8)]
    #[byte_size(16)]
    pub arr: Box<[Partial]>,
}
impl Default for Holder {
    fn default() -> Self {
        {
            Holder {
                p: <Partial>::default(),
                arr: (0..2)
                    .map(|_| <Partial>::default())
                    .collect::<Box<[Partial]>>(),
            }
        }
    }
}
thread_local!(
    pub static copies_1: Value<()> = Rc::new(RefCell::new(0_()));
);
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CopyAssign, Default)]
#[byte_size(4)]
pub struct CopyCounted {
    #[offset(0)]
    pub v: (),
}
impl CopyCounted {
    pub fn new(mut v: ()) -> Self {
        Self { v: v }
    }
    pub fn copy_from(o: Ptr<CopyCounted>) -> Self {
        let __this: CopyCounted = Self {
            v: o.with(|__s| __s.v),
        };
        (*copies_1.with(Value::clone).borrow_mut()).prefix_inc();
        __this
    }
}
impl Clone for CopyCounted {
    fn clone(&self) -> Self {
        let __src: Value<CopyCounted> = Rc::new(RefCell::new(CopyCounted { v: self.v.clone() }));
        CopyCounted::copy_from(__src.as_pointer())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CopyAssign)]
#[byte_size(12)]
pub struct HasCopyCounted {
    #[offset(0)]
    #[byte_size(4)]
    pub c: CopyCounted,
    #[offset(4)]
    #[byte_size(8)]
    pub arr: Box<[CopyCounted]>,
}
impl Default for HasCopyCounted {
    fn default() -> Self {
        {
            HasCopyCounted {
                c: <CopyCounted>::default(),
                arr: (0..2)
                    .map(|_| <CopyCounted>::default())
                    .collect::<Box<[CopyCounted]>>(),
            }
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 1_() }, { 100_() })));
    let b: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 2_() }, { 200_() })));
    let c: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 3_() }, { 300_() })));
    ({ PartialImpl::copy_assign(&a.as_pointer(), b.as_pointer()) });
    unimplemented!();
    unimplemented!();
    ({
        PartialImpl::copy_assign(
            &c.as_pointer(),
            ({ PartialImpl::copy_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    unimplemented!();
    unimplemented!();
    ({
        let _o: Ptr<Partial> = a.as_pointer();
        PartialImpl::copy_assign(&a.as_pointer(), _o)
    });
    unimplemented!();
    ({
        let _o: Value<Partial> = Rc::new(RefCell::new(Partial::new({ 9_() }, { 900_() })));
        PartialImpl::copy_assign(&a.as_pointer(), _o.as_pointer())
    });
    unimplemented!();
    unimplemented!();
    let ra: Ptr<Partial> = a.as_pointer();
    ({
        let _o: Ptr<Partial> = c.as_pointer();
        PartialImpl::copy_assign(&ra, _o)
    });
    unimplemented!();
    let mut pa: Ptr<Partial> = (a.as_pointer());
    ({
        let _o: Ptr<Partial> = b.as_pointer();
        PartialImpl::copy_assign(&pa, _o)
    });
    unimplemented!();
    unimplemented!();
    let h: Value<Holder> = Rc::new(RefCell::new(Holder {
        p: Partial::new({ 4_() }, { 40_() }),
        arr: Box::new([
            Partial::new({ 5_() }, { 50_() }),
            Partial::new({ 6_() }, { 60_() }),
        ]),
    }));
    ({ PartialImpl::copy_assign(&field_ptr!(h.as_pointer(), p), b.as_pointer()) });
    ({
        PartialImpl::copy_assign(
            &(array_field_ptr!(h.as_pointer(), arr) as Ptr<Partial>).offset((1_()) as isize),
            c.as_pointer(),
        )
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let n: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    let n1: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    let n2: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    let cn: Value<NonConstAssign> = Rc::new(RefCell::new(NonConstAssign::new()));
    ({ NonConstAssignImpl::operator_assign_2(&n1.as_pointer(), n.as_pointer()) });
    ({ NonConstAssignImpl::operator_assign_3(&n2.as_pointer(), cn.as_pointer()) });
    unimplemented!();
    unimplemented!();
    let r: Value<RefQualified> = Rc::new(RefCell::new(RefQualified::new()));
    let r1: Value<RefQualified> = Rc::new(RefCell::new(RefQualified::new()));
    ({ RefQualifiedImpl::copy_assign(&r1.as_pointer(), r.as_pointer()) });
    unimplemented!();
    let h2: Value<Holder> = Rc::new(RefCell::new(Holder {
        p: Partial::new({ 7_() }, { 70_() }),
        arr: Box::new([
            Partial::new({ 8_() }, { 80_() }),
            Partial::new({ 9_() }, { 90_() }),
        ]),
    }));
    ({ HolderImpl::copy_assign(&h2.as_pointer(), h.as_pointer()) });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let cc1: Value<CopyCounted> = Rc::new(RefCell::new(CopyCounted::new({ 1_() })));
    let cc2: Value<CopyCounted> = Rc::new(RefCell::new(CopyCounted::new({ 2_() })));
    ({ CopyCountedImpl::copy_assign(&cc1.as_pointer(), cc2.as_pointer()) });
    unimplemented!();
    let hc1: Value<HasCopyCounted> = Rc::new(RefCell::new(HasCopyCounted {
        c: CopyCounted::new({ 1_() }),
        arr: Box::new([CopyCounted::new({ 2_() }), CopyCounted::new({ 3_() })]),
    }));
    let hc2: Value<HasCopyCounted> = Rc::new(RefCell::new(HasCopyCounted {
        c: CopyCounted::new({ 4_() }),
        arr: Box::new([CopyCounted::new({ 5_() }), CopyCounted::new({ 6_() })]),
    }));
    ({ HasCopyCountedImpl::copy_assign(&hc1.as_pointer(), hc2.as_pointer()) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait CopyCountedImpl {
    fn copy_assign(&self, _a0: Ptr<CopyCounted>) -> Ptr<CopyCounted>;
}
impl CopyCountedImpl for Ptr<CopyCounted> {
    fn copy_assign(&self, _a0: Ptr<CopyCounted>) -> Ptr<CopyCounted> {
        field!((*self), v).write({ _a0.with(|__s| __s.v) });
        return (*self).clone();
    }
}
pub trait HasCopyCountedImpl {
    fn copy_assign(&self, _a0: Ptr<HasCopyCounted>) -> Ptr<HasCopyCounted>;
}
impl HasCopyCountedImpl for Ptr<HasCopyCounted> {
    fn copy_assign(&self, _a0: Ptr<HasCopyCounted>) -> Ptr<HasCopyCounted> {
        ({
            let _arg0: Ptr<CopyCounted> = field_ptr!(_a0, c);
            CopyCountedImpl::copy_assign(&field_ptr!((*self), c), _arg0)
        });
        unimplemented!();
        return (*self).clone();
    }
}
pub trait HolderImpl {
    fn copy_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder>;
}
impl HolderImpl for Ptr<Holder> {
    fn copy_assign(&self, _a0: Ptr<Holder>) -> Ptr<Holder> {
        ({
            let _o: Ptr<Partial> = field_ptr!(_a0, p);
            PartialImpl::copy_assign(&field_ptr!((*self), p), _o)
        });
        let mut __i0: () = 0_();
        'loop_: while (__i0 != 2_()) {
            ({
                let _o: Ptr<Partial> =
                    (array_field_ptr!(_a0, arr) as Ptr<Partial>).offset((__i0) as isize);
                PartialImpl::copy_assign(
                    &(array_field_ptr!((*self), arr) as Ptr<Partial>).offset((__i0) as isize),
                    _o,
                )
            });
            __i0.prefix_inc();
        }
        return (*self).clone();
    }
}
pub trait NonConstAssignImpl {
    fn operator_assign_2(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
    fn operator_assign_3(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign>;
}
impl NonConstAssignImpl for Ptr<NonConstAssign> {
    fn operator_assign_2(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1_()) });
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<NonConstAssign>) -> Ptr<NonConstAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 10_()) });
        return (*self).clone();
    }
}
pub trait PartialImpl {
    fn copy_assign(&self, o: Ptr<Partial>) -> Ptr<Partial>;
}
impl PartialImpl for Ptr<Partial> {
    fn copy_assign(&self, o: Ptr<Partial>) -> Ptr<Partial> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), v).write({ o.with(|__s| __s.v) });
        (*assigns_0.with(Value::clone).borrow_mut()).prefix_inc();
        return (*self).clone();
    }
}
pub trait RefQualifiedImpl {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified>;
}
impl RefQualifiedImpl for Ptr<RefQualified> {
    fn copy_assign(&self, o: Ptr<RefQualified>) -> Ptr<RefQualified> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1_()) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = assigns_0.with(|_| ());
    let _ = copies_1.with(|_| ());
}
