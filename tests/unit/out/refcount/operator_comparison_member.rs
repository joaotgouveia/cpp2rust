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
pub struct S {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::Ord for S {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if SImpl::operator_lt_3(
                &Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if SImpl::operator_lt_3(
                &Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for S {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        {
            SImpl::operator_eq(
                &Rc::new(RefCell::new(S { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(S { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for S {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let a: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
    let b: Value<S> = Rc::new(RefCell::new(S { v: 2_() }));
    let c: Value<S> = Rc::new(RefCell::new(S { v: 1_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub trait SImpl {
    fn operator_eq(&self, o: Ptr<S>) -> ();
    fn operator_ne(&self, o: Ptr<S>) -> ();
    fn operator_lt_3(&self, o: Ptr<S>) -> ();
    fn operator_gt(&self, o: Ptr<S>) -> ();
    fn operator_le(&self, o: Ptr<S>) -> ();
    fn operator_ge(&self, o: Ptr<S>) -> ();
    fn operator_lt_7(&self, o: ()) -> ();
}
impl SImpl for Ptr<S> {
    fn operator_eq(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) });
    }
    fn operator_ne(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } != { o.with(|__s| __s.v) });
    }
    fn operator_lt_3(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) });
    }
    fn operator_gt(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } > { o.with(|__s| __s.v) });
    }
    fn operator_le(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } <= { o.with(|__s| __s.v) });
    }
    fn operator_ge(&self, o: Ptr<S>) -> () {
        return ({ (*self).with(|__s| __s.v) } >= { o.with(|__s| __s.v) });
    }
    fn operator_lt_7(&self, mut o: ()) -> () {
        return ((*self).with(|__s| __s.v) < o);
    }
}
pub fn __cpp2rust_init_globals() {}
