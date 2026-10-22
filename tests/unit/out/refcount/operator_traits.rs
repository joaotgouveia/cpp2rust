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
impl std::cmp::Ord for Lt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Lt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Lt {
    fn eq(&self, other: &Self) -> bool {
        {
            !(LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
            )) && !(LtImpl::operator_lt(
                &Rc::new(RefCell::new(Lt { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Lt { v: self.v.clone() })).as_pointer(),
            ))
        }
    }
}
impl std::cmp::Eq for Lt {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Eq {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::PartialEq for Eq {
    fn eq(&self, other: &Self) -> bool {
        {
            EqImpl::operator_eq(
                &Rc::new(RefCell::new(Eq { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Eq { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Eq {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Cmp {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::Ord for Cmp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            CmpImpl::operator_cmp(
                &Rc::new(RefCell::new(Cmp { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Cmp { v: other.v.clone() })).as_pointer(),
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
            CmpImpl::operator_eq(
                &Rc::new(RefCell::new(Cmp { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Cmp { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Cmp {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Free {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::Ord for Free {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_0(
                Rc::new(RefCell::new(Free { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Free { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(
                Rc::new(RefCell::new(Free { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Free { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Free {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Free {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_1(
                Rc::new(RefCell::new(Free { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Free { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Free {}
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
