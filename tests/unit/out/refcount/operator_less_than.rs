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
pub struct Pair {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
}
impl CppLess for Pair {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        PairImpl::operator_lt(__lhs, __rhs.clone())
    }
}
impl CppEq for Pair {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        !(PairImpl::operator_lt(__lhs, __rhs.clone()))
            && !(PairImpl::operator_lt(__rhs, __lhs.clone()))
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let pair1: Value<Pair> = Rc::new(RefCell::new(Pair { x: 1_(), y: 2_() }));
    let pair2: Value<Pair> = Rc::new(RefCell::new(Pair { x: 1_(), y: 3_() }));
    unimplemented!();
    return 0_();
}
pub trait PairImpl {
    fn operator_lt(&self, other: Ptr<Pair>) -> ();
}
impl PairImpl for Ptr<Pair> {
    fn operator_lt(&self, other: Ptr<Pair>) -> () {
        return ({ (*self).with(|__s| __s.x) } < { other.with(|__s| __s.x) })
            || (({ (*self).with(|__s| __s.x) } == { other.with(|__s| __s.x) })
                && ({ (*self).with(|__s| __s.y) } < { other.with(|__s| __s.y) }));
    }
}
pub fn __cpp2rust_init_globals() {}
