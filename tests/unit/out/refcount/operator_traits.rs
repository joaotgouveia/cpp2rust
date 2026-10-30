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
pub struct Lt {
    #[offset(0)]
    pub v: (),
}
impl CppLess for Lt {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        LtImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Lt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(LtImpl::operator_lt(__lhs, __rhs.clone())) && !(LtImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Eq {
    #[offset(0)]
    pub v: (),
}
impl CppEq for Eq {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        EqImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Cmp {
    #[offset(0)]
    pub v: (),
}
impl CppLess for Cmp {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for Cmp {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        CmpImpl::operator_eq(__lhs, __rhs.clone())
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Free {
    #[offset(0)]
    pub v: (),
}
impl CppLess for Free {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_lt_0(__lhs.clone(), __rhs.clone())
    }
}
impl CppEq for Free {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_1(__lhs.clone(), __rhs.clone())
    }
}
pub fn operator_lt_0(a: Ptr<Free>, b: Ptr<Free>) -> () {
    return ({ a.with(|__s| __s.v) } < { b.with(|__s| __s.v) });
}
pub fn operator_eq_1(a: Ptr<Free>, b: Ptr<Free>) -> () {
    return ({ a.with(|__s| __s.v) } == { b.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Wrapped_int_ {
    #[offset(0)]
    pub v: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut w: Wrapped_int_ = Wrapped_int_ { v: 2_() };
    unimplemented!();
    let mut lts: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut eqs: () = unimplemented!();
    let mut two: Eq = Eq { v: 2_() };
    let mut nine: Eq = Eq { v: 9_() };
    unimplemented!();
    unimplemented!();
    let mut cmps: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut three: Cmp = Cmp { v: 3_() };
    unimplemented!();
    let mut frees: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut ftwo: Free = Free { v: 2_() };
    unimplemented!();
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!().write(20_());
    unimplemented!().write(10_());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub trait CmpImpl {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> ();
    fn operator_eq(&self, o: Ptr<Cmp>) -> ();
}
impl CmpImpl for Ptr<Cmp> {
    fn operator_cmp(&self, o: Ptr<Cmp>) -> () {
        return std::cmp::Ord::cmp(&((*self).with(|__s| __s.v)), &(o.with(|__s| __s.v)));
    }
    fn operator_eq(&self, o: Ptr<Cmp>) -> () {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait EqImpl {
    fn operator_eq(&self, o: Ptr<Eq>) -> ();
}
impl EqImpl for Ptr<Eq> {
    fn operator_eq(&self, o: Ptr<Eq>) -> () {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
}
pub trait LtImpl {
    fn operator_lt(&self, o: Ptr<Lt>) -> ();
}
impl LtImpl for Ptr<Lt> {
    fn operator_lt(&self, o: Ptr<Lt>) -> () {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
}
pub fn __cpp2rust_init_globals() {}
