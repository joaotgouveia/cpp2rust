extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_eq_0(_a0: Ptr<Defaulted>, _a1: Ptr<Defaulted>) -> () {
    return ({ _a0.with(|__s| __s.a) } == { _a1.with(|__s| __s.a) })
        && ({ _a0.with(|__s| __s.b) } == { _a1.with(|__s| __s.b) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Defaulted {
    #[offset(0)]
    pub a: (),
    #[offset(4)]
    pub b: (),
}
impl std::cmp::PartialEq for Defaulted {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_0(
                Rc::new(RefCell::new(Defaulted {
                    a: self.a.clone(),
                    b: self.b.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(Defaulted {
                    a: other.a.clone(),
                    b: other.b.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Defaulted {}
pub fn operator_cmp_1(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> () {
    {
        let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
            &(_a0.with(|__s| __s.a)),
            &(_a1.with(|__s| __s.a)),
        )));
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
pub fn operator_eq_2(_a0: Ptr<DefaultedOrd>, _a1: Ptr<DefaultedOrd>) -> () {
    return ({ _a0.with(|__s| __s.a) } == { _a1.with(|__s| __s.a) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DefaultedOrd {
    #[offset(0)]
    pub a: (),
}
impl std::cmp::Ord for DefaultedOrd {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            operator_cmp_1(
                Rc::new(RefCell::new(DefaultedOrd { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(DefaultedOrd { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for DefaultedOrd {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for DefaultedOrd {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_2(
                Rc::new(RefCell::new(DefaultedOrd { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(DefaultedOrd { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for DefaultedOrd {}
pub fn operator_eq_3(x: Ptr<Inline>, y: Ptr<Inline>) -> () {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) });
}
pub fn operator_lt_4(x: Ptr<Inline>, y: Ptr<Inline>) -> () {
    return ({ x.with(|__s| __s.a) } < { y.with(|__s| __s.a) });
}
pub fn operator_add_5(x: Ptr<Inline>, y: Ptr<Inline>) -> Inline {
    return Inline {
        a: ({ x.with(|__s| __s.a) } + { y.with(|__s| __s.a) }),
    };
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Inline {
    #[offset(0)]
    pub a: (),
}
impl std::cmp::Ord for Inline {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_4(
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Inline {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Inline {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_3(
                Rc::new(RefCell::new(Inline { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(Inline { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Inline {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct OutOfLine {
    #[offset(0)]
    pub a: (),
}
impl std::cmp::PartialEq for OutOfLine {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_6(
                Rc::new(RefCell::new(OutOfLine { a: self.a.clone() })).as_pointer(),
                Rc::new(RefCell::new(OutOfLine { a: other.a.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for OutOfLine {}
pub fn operator_eq_6(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> () {
    return ({ x.with(|__s| __s.a) } == { y.with(|__s| __s.a) });
}
pub fn operator_ne_7(x: Ptr<OutOfLine>, y: Ptr<OutOfLine>) -> () {
    return !({
        let _x: Ptr<OutOfLine> = (x).clone();
        let _y: Ptr<OutOfLine> = (y).clone();
        operator_eq_6(_x, _y)
    });
}
pub fn operator_eq_8(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_int_>) -> () {
    return ({ x.with(|__s| __s.v) } == { y.with(|__s| __s.v) });
}
pub fn operator_lt_9(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_int_>) -> () {
    return ({ x.with(|__s| __s.v) } < { y.with(|__s| __s.v) });
}
pub fn operator_eq_10(x: Ptr<Tmpl_int_>, y: Ptr<Tmpl_long_>) -> () {
    return ({ x.with(|__s| __s.v) } == { y.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Tmpl_int_ {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::Ord for Tmpl_int_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            if operator_lt_9(
                Rc::new(RefCell::new(Tmpl_int_ { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_int_ { v: other.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_9(
                Rc::new(RefCell::new(Tmpl_int_ { v: other.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_int_ { v: self.v.clone() })).as_pointer(),
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Tmpl_int_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Tmpl_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_8(
                Rc::new(RefCell::new(Tmpl_int_ { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(Tmpl_int_ { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for Tmpl_int_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Tmpl_long_ {
    #[offset(0)]
    pub v: (),
}
pub fn operator_eq_11(_a0: Ptr<TmplDefaulted_int_>, _a1: Ptr<TmplDefaulted_int_>) -> () {
    return ({ _a0.with(|__s| __s.v) } == { _a1.with(|__s| __s.v) });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct TmplDefaulted_int_ {
    #[offset(0)]
    pub v: (),
}
impl std::cmp::PartialEq for TmplDefaulted_int_ {
    fn eq(&self, other: &Self) -> bool {
        {
            operator_eq_11(
                Rc::new(RefCell::new(TmplDefaulted_int_ { v: self.v.clone() })).as_pointer(),
                Rc::new(RefCell::new(TmplDefaulted_int_ { v: other.v.clone() })).as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for TmplDefaulted_int_ {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let d1: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1_(), b: 2_() }));
    let d2: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1_(), b: 2_() }));
    let d3: Value<Defaulted> = Rc::new(RefCell::new(Defaulted { a: 1_(), b: 3_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let o1: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 1_() }));
    let o2: Value<DefaultedOrd> = Rc::new(RefCell::new(DefaultedOrd { a: 2_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let i1: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1_() }));
    let i2: Value<Inline> = Rc::new(RefCell::new(Inline { a: 2_() }));
    let i3: Value<Inline> = Rc::new(RefCell::new(Inline { a: 1_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let f1: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4_() }));
    let f2: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 4_() }));
    let f3: Value<OutOfLine> = Rc::new(RefCell::new(OutOfLine { a: 5_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let t1: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 1_() }));
    let t2: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 2_() }));
    let t3: Value<Tmpl_int_> = Rc::new(RefCell::new(Tmpl_int_ { v: 1_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let u1: Value<Tmpl_long_> = Rc::new(RefCell::new(Tmpl_long_ { v: 1_() }));
    let u2: Value<Tmpl_long_> = Rc::new(RefCell::new(Tmpl_long_ { v: 2_() }));
    unimplemented!();
    unimplemented!();
    let v1: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 7_() }));
    let v2: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 7_() }));
    let v3: Value<TmplDefaulted_int_> = Rc::new(RefCell::new(TmplDefaulted_int_ { v: 8_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
