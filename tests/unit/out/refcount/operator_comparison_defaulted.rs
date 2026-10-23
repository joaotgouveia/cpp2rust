extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Eq {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        {
            EqImpl::operator_eq(
                &Rc::new(RefCell::new(Eq {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Eq {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Eq {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cmp {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            CmpImpl::operator_cmp(
                &Rc::new(RefCell::new(Cmp {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Cmp {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Cmp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Cmp {
    fn eq(&self, other: &Self) -> bool {
        {
            CmpImpl::operator_cmp(
                &Rc::new(RefCell::new(Cmp {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Cmp {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for Cmp {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Both {
    #[offset(0)]
    pub a: (),
}
impl std::cmp::Ord for Both {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            BothImpl::operator_cmp(
                &Rc::new(RefCell::new(Both { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Both { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Both {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Both {
    fn eq(&self, other: &Self) -> bool {
        {
            BothImpl::operator_eq(
                &Rc::new(RefCell::new(Both { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Both { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Both {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OrdOnly {
    #[offset(0)]
    pub a: (),
}
impl std::cmp::Ord for OrdOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            OrdOnlyImpl::operator_cmp(
                &Rc::new(RefCell::new(OrdOnly { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(OrdOnly { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for OrdOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for OrdOnly {
    fn eq(&self, other: &Self) -> bool {
        {
            OrdOnlyImpl::operator_cmp(
                &Rc::new(RefCell::new(OrdOnly { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(OrdOnly { a: other.a.clone() })).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for OrdOnly {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inner {
    #[offset(0)]
    pub x: (),
}
impl std::cmp::Ord for Inner {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            InnerImpl::operator_cmp(
                &Rc::new(RefCell::new(Inner { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inner { x: other.x.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Inner {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inner {
    fn eq(&self, other: &Self) -> bool {
        {
            InnerImpl::operator_cmp(
                &Rc::new(RefCell::new(Inner { x: self.x.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inner { x: other.x.clone() })).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for Inner {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Outer {
    #[offset(0)]
    #[byte_size(4)]
    pub i: Inner,
    #[offset(4)]
    pub y: (),
}
impl std::cmp::Ord for Outer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            OuterImpl::operator_cmp(
                &Rc::new(RefCell::new(Outer {
                    i: Inner {
                        x: self.i.x.clone(),
                    },
                    y: self.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Outer {
                    i: Inner {
                        x: other.i.x.clone(),
                    },
                    y: other.y.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Outer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Outer {
    fn eq(&self, other: &Self) -> bool {
        {
            OuterImpl::operator_cmp(
                &Rc::new(RefCell::new(Outer {
                    i: Inner {
                        x: self.i.x.clone(),
                    },
                    y: self.y.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Outer {
                    i: Inner {
                        x: other.i.x.clone(),
                    },
                    y: other.y.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for Outer {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Secondary {
    #[offset(0)]
    pub a: (),
}
impl std::cmp::Ord for Secondary {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            SecondaryImpl::operator_cmp(
                &Rc::new(RefCell::new(Secondary { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Secondary { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for Secondary {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Secondary {
    fn eq(&self, other: &Self) -> bool {
        {
            SecondaryImpl::operator_eq(
                &Rc::new(RefCell::new(Secondary { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Secondary { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Secondary {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct PtrMember {
    #[offset(0)]
    #[byte_size(8)]
    pub p: Ptr<()>,
}
impl std::cmp::Ord for PtrMember {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            PtrMemberImpl::operator_cmp(
                &Rc::new(RefCell::new(PtrMember { p: self.p.clone() })).as_pointer(),
                Rc::new(RefCell::new(PtrMember { p: other.p.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for PtrMember {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PtrMember {
    fn eq(&self, other: &Self) -> bool {
        {
            PtrMemberImpl::operator_cmp(
                &Rc::new(RefCell::new(PtrMember { p: self.p.clone() })).as_pointer(),
                Rc::new(RefCell::new(PtrMember { p: other.p.clone() })).as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for PtrMember {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let e1: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1_(), b: 2_() }));
    let e2: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1_(), b: 2_() }));
    let e3: Value<Eq> = Rc::new(RefCell::new(Eq { a: 1_(), b: 3_() }));
    unimplemented!();
    unimplemented!();
    let c1: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1_(), b: 2_() }));
    let c2: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1_(), b: 3_() }));
    let c3: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 2_(), b: 0_() }));
    let c4: Value<Cmp> = Rc::new(RefCell::new(Cmp { a: 1_(), b: 9_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let b1: Value<Both> = Rc::new(RefCell::new(Both { a: 1_() }));
    let b2: Value<Both> = Rc::new(RefCell::new(Both { a: 2_() }));
    unimplemented!();
    unimplemented!();
    let o1: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 1_() }));
    let o2: Value<OrdOnly> = Rc::new(RefCell::new(OrdOnly { a: 2_() }));
    unimplemented!();
    unimplemented!();
    let x1: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1_() },
        y: 9_(),
    }));
    let x2: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 2_() },
        y: 0_(),
    }));
    let x3: Value<Outer> = Rc::new(RefCell::new(Outer {
        i: Inner { x: 1_() },
        y: 9_(),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let s1: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 1_() }));
    let s2: Value<Secondary> = Rc::new(RefCell::new(Secondary { a: 2_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let arr: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([0_(), 0_()])));
    let p1: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<()>),
    }));
    let p2: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<()>).offset((1_()) as isize),
    }));
    let p3: Value<PtrMember> = Rc::new(RefCell::new(PtrMember {
        p: (arr.as_pointer() as Ptr<()>),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait BothImpl {
    fn operator_eq(&self, _a0: Ptr<Both>) -> ();
    fn operator_cmp(&self, _a0: Ptr<Both>) -> ();
}
impl BothImpl for Ptr<Both> {
    fn operator_eq(&self, _a0: Ptr<Both>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_cmp(&self, _a0: Ptr<Both>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
}
pub trait CmpImpl {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> ();
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> ();
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, _a0: Ptr<Cmp>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.b)),
                &(_a0.with(|__s| __s.b)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<Cmp>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> ();
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, _a0: Ptr<Eq>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) })
            && ({ (*self).with(|__s| __s.b) } == { _a0.with(|__s| __s.b) });
    }
}
pub trait InnerImpl {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> ();
    fn operator_eq(&self, _a0: Ptr<Inner>) -> ();
}
impl InnerImpl for Ptr<Inner> {
    fn operator_cmp(&self, _a0: Ptr<Inner>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.x)),
                &(_a0.with(|__s| __s.x)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<Inner>) -> () {
        return ({ (*self).with(|__s| __s.x) } == { _a0.with(|__s| __s.x) });
    }
}
pub trait OrdOnlyImpl {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> ();
}
impl OrdOnlyImpl for Ptr<OrdOnly> {
    fn operator_cmp(&self, _a0: Ptr<OrdOnly>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
}
pub trait OuterImpl {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> ();
    fn operator_eq(&self, _a0: Ptr<Outer>) -> ();
}
impl OuterImpl for Ptr<Outer> {
    fn operator_cmp(&self, _a0: Ptr<Outer>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(
                ({
                    let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
                    InnerImpl::operator_cmp(&field_ptr!((*self), i), _arg0)
                }),
            ));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.y)),
                &(_a0.with(|__s| __s.y)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<Outer>) -> () {
        return ({
            let _arg0: Ptr<Inner> = field_ptr!(_a0, i);
            InnerImpl::operator_eq(&field_ptr!((*self), i), _arg0)
        }) && ({ (*self).with(|__s| __s.y) } == { _a0.with(|__s| __s.y) });
    }
}
pub trait PtrMemberImpl {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> ();
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> ();
}
impl PtrMemberImpl for Ptr<PtrMember> {
    fn operator_cmp(&self, _a0: Ptr<PtrMember>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.p.clone())),
                &(_a0.with(|__s| __s.p.clone())),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_eq(&self, _a0: Ptr<PtrMember>) -> () {
        return ({ (*self).with(|__s| __s.p.clone()) } == { _a0.with(|__s| __s.p.clone()) });
    }
}
pub trait SecondaryImpl {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> ();
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> ();
}
impl SecondaryImpl for Ptr<Secondary> {
    fn operator_eq(&self, _a0: Ptr<Secondary>) -> () {
        return ({ (*self).with(|__s| __s.a) } == { _a0.with(|__s| __s.a) });
    }
    fn operator_ne(&self, _a0: Ptr<Secondary>) -> () {
        return !({
            let _arg0: Ptr<Secondary> = (_a0).clone();
            SecondaryImpl::operator_eq(&(*self), _arg0)
        });
    }
    fn operator_cmp(&self, _a0: Ptr<Secondary>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.a)),
                &(_a0.with(|__s| __s.a)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_lt(&self, _a0: Ptr<Secondary>) -> () {
        return unimplemented!();
    }
    fn operator_ge(&self, _a0: Ptr<Secondary>) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
