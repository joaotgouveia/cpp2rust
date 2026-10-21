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
pub struct X {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::Ord for X {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if ((operator_lt_0(self.clone(), other.clone())) != 0) {
                std::cmp::Ordering::Less
            } else if ((operator_lt_0(other.clone(), self.clone())) != 0) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for X {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        { ((operator_eq_1(self.clone(), other.clone())) != 0) }
    }
}
impl std::cmp::Eq for X {}
pub fn operator_eq_1(mut a: X, mut b: X) -> () {
    return if (a.v == b.v) { 2_() } else { unimplemented!() };
}
pub fn operator_ne_2(mut a: X, mut b: X) -> () {
    return if (a.v != b.v) { 3_() } else { unimplemented!() };
}
pub fn operator_lt_0(mut a: X, mut b: X) -> () {
    return if (a.v < b.v) { 4_() } else { unimplemented!() };
}
pub fn operator_gt_3(mut a: X, mut b: X) -> () {
    return if (a.v > b.v) { 5_() } else { unimplemented!() };
}
pub fn operator_le_4(mut a: X, mut b: X) -> () {
    return if (a.v <= b.v) { 6_() } else { unimplemented!() };
}
pub fn operator_ge_5(mut a: X, mut b: X) -> () {
    return if (a.v >= b.v) { 7_() } else { unimplemented!() };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Result {
    #[offset(0)]
    pub r: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Custom {
    #[offset(0)]
    pub v: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Mixed {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::Ord for Mixed {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if ((MixedImpl::operator_lt(
                &Rc::new(RefCell::new(Mixed { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Mixed { v: other.v.clone() })).as_pointer(),
            )) != 0)
            {
                std::cmp::Ordering::Less
            } else if ((MixedImpl::operator_lt(
                &Rc::new(RefCell::new(Mixed { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Mixed { v: self.v.clone() })).as_pointer(),
            )) != 0)
            {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Mixed {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Mixed {
    fn eq(&self, other: &Self) -> bool {
        {
            ((MixedImpl::operator_eq(
                &Rc::new(RefCell::new(Mixed { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Mixed { v: other.v.clone() })).as_pointer(),
            )) != 0)
        }
    }
}
impl std::cmp::Eq for Mixed {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct MyBool {
    #[offset(0)]
    pub value: (),
}
impl MyBool {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Boolish {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::Ord for Boolish {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if {
                let __cmp = &operator_lt_6(self.clone(), other.clone());
                MyBoolImpl::to_(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            } {
                std::cmp::Ordering::Less
            } else if {
                let __cmp = &operator_lt_6(other.clone(), self.clone());
                MyBoolImpl::to_(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            } {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Boolish {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Boolish {
    fn eq(&self, other: &Self) -> bool {
        {
            {
                let __cmp = &operator_eq_7(self.clone(), other.clone());
                MyBoolImpl::to_(
                    &Rc::new(RefCell::new(MyBool {
                        value: __cmp.value.clone(),
                    }))
                    .as_pointer(),
                )
            }
        }
    }
}
impl std::cmp::Eq for Boolish {}
pub fn operator_eq_7(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ (a.v == b.v) });
}
pub fn operator_lt_6(mut a: Boolish, mut b: Boolish) -> MyBool {
    return MyBool::new({ (a.v < b.v) });
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct RefBool {
    #[offset(0)]
    pub value: (),
}
impl RefBool {
    pub fn new(mut v: ()) -> Self {
        Self { value: v }
    }
}
thread_local!(
    pub static ref_true_8: Value<RefBool> = Rc::new(RefCell::new(RefBool::new({ true })));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct RefInt {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::PartialEq for RefInt {
    fn eq(&self, other: &Self) -> bool {
        {
            {
                let __cmp = RefIntImpl::operator_eq(
                    &Rc::new(RefCell::new(RefInt { v: self.v.clone() })).as_pointer(),
                    Rc::new(RefCell::new(RefInt { v: other.v.clone() })).as_pointer(),
                );
                RefBoolImpl::to_(&__cmp)
            }
        }
    }
}
impl std::cmp::Eq for RefInt {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let mut a: X = X { v: 1_() };
    let mut b: X = X { v: 2_() };
    let mut c: X = X { v: 1_() };
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let xs: Value<Box<[X]>> = Rc::new(RefCell::new(Box::new([
        X { v: 3_() },
        X { v: 1_() },
        X { v: 2_() },
    ])));
    unimplemented!();
    unimplemented!();
    let p: Value<Custom> = Rc::new(RefCell::new(Custom { v: 5_() }));
    let q: Value<Custom> = Rc::new(RefCell::new(Custom { v: 2_() }));
    let mut r: Result = ({ CustomImpl::operator_cmp(&p.as_pointer(), q.as_pointer()) });
    unimplemented!();
    let m: Value<Mixed> = Rc::new(RefCell::new(Mixed { v: 4_() }));
    let n: Value<Mixed> = Rc::new(RefCell::new(Mixed { v: 4_() }));
    unimplemented!();
    unimplemented!();
    let ms: Value<Box<[Mixed]>> = Rc::new(RefCell::new(Box::new([
        Mixed { v: 6_() },
        Mixed { v: 4_() },
        Mixed { v: 5_() },
    ])));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b1: Boolish = Boolish { v: 1_() };
    let mut b2: Boolish = Boolish { v: 2_() };
    unimplemented!();
    unimplemented!();
    let mut lt: MyBool = ({
        let _a: Boolish = (b1).clone();
        operator_lt_6(_a, (b2).clone())
    });
    unimplemented!();
    let bs: Value<Box<[Boolish]>> = Rc::new(RefCell::new(Box::new([
        Boolish { v: 3_() },
        Boolish { v: 1_() },
        Boolish { v: 2_() },
    ])));
    unimplemented!();
    unimplemented!();
    let ri1: Value<RefInt> = Rc::new(RefCell::new(RefInt { v: 1_() }));
    let ri2: Value<RefInt> = Rc::new(RefCell::new(RefInt { v: 2_() }));
    unimplemented!();
    let ris: Value<Box<[RefInt]>> = Rc::new(RefCell::new(Box::new([
        RefInt { v: 1_() },
        RefInt { v: 2_() },
    ])));
    unimplemented!();
    return unimplemented!();
}
pub trait CustomImpl {
    fn operator_cmp(&self, o: Ptr<Custom>) -> Result;
}
impl CustomImpl for Ptr<Custom> {
    fn operator_cmp(&self, o: Ptr<Custom>) -> Result {
        return Result {
            r: ({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }),
        };
    }
}
pub trait MixedImpl {
    fn operator_cmp(&self, o: Ptr<Mixed>) -> Result;
    fn operator_lt(&self, o: Ptr<Mixed>) -> ();
    fn operator_eq(&self, o: Ptr<Mixed>) -> ();
}
impl MixedImpl for Ptr<Mixed> {
    fn operator_cmp(&self, o: Ptr<Mixed>) -> Result {
        return Result {
            r: ({ (*self).with(|__s| __s.v) } - { o.with(|__s| __s.v) }),
        };
    }
    fn operator_lt(&self, o: Ptr<Mixed>) -> () {
        return if ({ (*self).with(|__s| __s.v) } < { o.with(|__s| __s.v) }) {
            8_()
        } else {
            unimplemented!()
        };
    }
    fn operator_eq(&self, o: Ptr<Mixed>) -> () {
        return if ({ (*self).with(|__s| __s.v) } == { o.with(|__s| __s.v) }) {
            9_()
        } else {
            unimplemented!()
        };
    }
}
pub trait MyBoolImpl {
    fn to_(&self) -> ();
}
impl MyBoolImpl for Ptr<MyBool> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefBoolImpl {
    fn to_(&self) -> ();
}
impl RefBoolImpl for Ptr<RefBool> {
    fn to_(&self) -> () {
        return (*self).with(|__s| __s.value);
    }
}
pub trait RefIntImpl {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool>;
}
impl RefIntImpl for Ptr<RefInt> {
    fn operator_eq(&self, _a0: Ptr<RefInt>) -> Ptr<RefBool> {
        return ref_true_8.with(|v| v.as_pointer());
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = ref_true_8.with(|_| ());
}
