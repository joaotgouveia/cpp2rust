extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_weak_ordering__std_weak_ordering_ {}
pub fn has_weak_order_1(t: Ptr<()>, u: Ptr<()>) -> () {
    return true;
}
pub fn has_weak_order_2(t: Ptr<Ptr<()>>, u: Ptr<Ptr<()>>) -> () {
    return true;
}
pub fn has_weak_order_3(t: Ptr<Ptr<()>>, u: Ptr<Ptr<()>>) -> () {
    return true;
}
pub fn has_weak_order_4(t: Ptr<N11_A>, u: Ptr<N11_A>) -> () {
    return true;
}
pub fn has_weak_order_5(__args: Box<[VaArg]>) -> () {
    let __args: Value<Box<[VaArg]>> = Rc::new(RefCell::new(__args.into()));
    return false;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N11_A {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N11_B {}
pub fn weak_order_6(_a0: Ptr<N11_A>, _a1: Ptr<N11_A>) -> () {
    return unimplemented!();
}
pub fn test_1_1_7() {
    const _: () = assert!(true, "has_weak_order(1, 2)");;
    const _: () = assert!(true, "!has_weak_order(1, (short)2)");;
    const _: () = assert!(true, "!has_weak_order(1, 2.0)");;
    const _: () = assert!(true, "!has_weak_order(1.0f, 2.0)");;
    const _: () = assert!(true, "has_weak_order((int*)nullptr, (int*)nullptr)");;
    const _: () = assert!(true, "!has_weak_order((int*)nullptr, (const int*)nullptr)");;
    const _: () = assert!(true, "!has_weak_order((const int*)nullptr, (int*)nullptr)");;
    const _: () = assert!(
        true,
        "has_weak_order((const int*)nullptr, (const int*)nullptr)"
    );;
    let a: Value<N11_A> = Rc::new(RefCell::new(<N11_A>::default()));
    let mut b: N11_B = <N11_B>::default();
    const _: () = assert!(true, "has_weak_order(a, a)");;
    const _: () = assert!(true, "!has_weak_order(a, b)");;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N12_A {}
pub fn weak_order_8(_a0: Ptr<N12_A>, _a1: Ptr<N12_A>) -> () {
    return unimplemented!();
}
pub fn weak_order_9(_a0: Ptr<N12_A>, _a1: Ptr<N12_A>) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N12_B {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N12_WeakOrder {}
pub fn weak_order_10(lhs: Ptr<N12_C>, _a1: Ptr<N12_C>) -> N12_WeakOrder {
    field!(lhs, touched).write(true);
    return <N12_WeakOrder>::default();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct N12_C {
    #[offset(0)]
    pub touched: (),
}
impl Default for N12_C {
    fn default() -> Self {
        { N12_C { touched: false } }
    }
}
pub fn test_1_2_11() {
    let a: Value<N12_A> = Rc::new(RefCell::new(<N12_A>::default()));
    unimplemented!();
    unimplemented!();
    let mut b: N12_B = <N12_B>::default();
    const _: () = assert!(true, "!has_weak_order(b, b)");;
    let c1: Value<N12_C> = Rc::new(RefCell::new(<N12_C>::default()));
    let c2: Value<N12_C> = Rc::new(RefCell::new(<N12_C>::default()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub fn test_1_3_12() -> () {
    const _: () = assert!(true, "");;
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        -unimplemented!(),
        unimplemented!(),
        -1_f64,
        -0.1_f64,
        -unimplemented!(),
        -0_f64,
        0_f64,
        unimplemented!(),
        0.1_f64,
        1_f64,
        2_f64,
        3.14_f64,
        unimplemented!(),
        unimplemented!(),
    ])));
    const _: () = assert!(true, "std::size(v) == 14");;
    let mut i: () = 0_();
    'loop_: while (i < 14_()) {
        let mut j: () = 0_();
        'loop_: while (j < 14_()) {
            let naturalOrder: Value<()> = Rc::new(RefCell::new(
                (std::cmp::Ord::cmp(
                    &((*v.borrow())[(i) as usize]),
                    &((*v.borrow())[(j) as usize]),
                )),
            ));
            if ((*v.borrow())[(i) as usize] == 0_()) && ((*v.borrow())[(j) as usize] == 0_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let nq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ns: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ps: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_1_3_13() -> () {
    const _: () = assert!(true, "");;
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        -unimplemented!(),
        unimplemented!(),
        -1_f64,
        -0.1_f64,
        -unimplemented!(),
        -0_f64,
        0_f64,
        unimplemented!(),
        0.1_f64,
        1_f64,
        2_f64,
        3.14_f64,
        unimplemented!(),
        unimplemented!(),
    ])));
    const _: () = assert!(true, "std::size(v) == 14");;
    let mut i: () = 0_();
    'loop_: while (i < 14_()) {
        let mut j: () = 0_();
        'loop_: while (j < 14_()) {
            let naturalOrder: Value<()> = Rc::new(RefCell::new(
                (std::cmp::Ord::cmp(
                    &((*v.borrow())[(i) as usize]),
                    &((*v.borrow())[(j) as usize]),
                )),
            ));
            if ((*v.borrow())[(i) as usize] == 0_()) && ((*v.borrow())[(j) as usize] == 0_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let nq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ns: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ps: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub fn test_1_3_14() -> () {
    const _: () = assert!(true, "");;
    let v: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
        -unimplemented!(),
        unimplemented!(),
        -1_f64,
        -0.1_f64,
        -unimplemented!(),
        -0_f64,
        0_f64,
        unimplemented!(),
        0.1_f64,
        1_f64,
        2_f64,
        3.14_f64,
        unimplemented!(),
        unimplemented!(),
    ])));
    const _: () = assert!(true, "std::size(v) == 14");;
    let mut i: () = 0_();
    'loop_: while (i < 14_()) {
        let mut j: () = 0_();
        'loop_: while (j < 14_()) {
            let naturalOrder: Value<()> = Rc::new(RefCell::new(
                (std::cmp::Ord::cmp(
                    &((*v.borrow())[(i) as usize]),
                    &((*v.borrow())[(j) as usize]),
                )),
            ));
            if ((*v.borrow())[(i) as usize] == 0_()) && ((*v.borrow())[(j) as usize] == 0_()) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let nq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ns: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let ps: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let pq: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut i: () = 0_();
        'loop_: while (i < 14_()) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
        unimplemented!();
    }
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N14_A {}
impl CppLess for N14_A {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_15(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for N14_A {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_cmp_15(__lhs.clone(), __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
pub fn operator_cmp_16(_a0: Ptr<N14_A>, _a1: Ptr<N14_A>) -> () {
    return unimplemented!();
}
pub fn operator_cmp_15(_a0: Ptr<N14_A>, _a1: Ptr<N14_A>) -> () {
    return unimplemented!();
}
const _: () = assert!(true, "std::three_way_comparable<A>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N14_B {}
const _: () = assert!(!(false), "!std::three_way_comparable<B>");
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct N14_C {
    #[offset(0)]
    #[byte_size(8)]
    pub touched: Ptr<()>,
}
impl CppLess for N14_C {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        N14_CImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for N14_C {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        N14_CImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
const _: () = assert!(true, "std::three_way_comparable<C>");
pub fn test_1_4_17() -> () {
    let a: Value<N14_A> = Rc::new(RefCell::new(<N14_A>::default()));
    unimplemented!();
    unimplemented!();
    let mut b: N14_B = <N14_B>::default();
    const _: () = assert!(true, "!has_weak_order(b, b)");;
    let c1_touched: Value<()> = Rc::new(RefCell::new(false));
    let c2_touched: Value<()> = Rc::new(RefCell::new(false));
    let c1: Value<N14_C> = Rc::new(RefCell::new(N14_C {
        touched: (c1_touched.as_pointer()),
    }));
    let c2: Value<N14_C> = Rc::new(RefCell::new(N14_C {
        touched: (c2_touched.as_pointer()),
    }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N15_A {}
pub fn strong_order_18(_a0: Ptr<N15_A>, _a1: Ptr<N15_A>) -> () {
    return unimplemented!();
}
pub fn strong_order_19(_a0: Ptr<N15_A>, _a1: Ptr<N15_A>) -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N15_B {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N15_WeakOrder {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N15_C {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct N15_StrongOrder {}
pub fn strong_order_20(lhs: Ptr<N15_D>, _a1: Ptr<N15_D>) -> N15_StrongOrder {
    field!(lhs, touched).write(true);
    return <N15_StrongOrder>::default();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct N15_D {
    #[offset(0)]
    pub touched: (),
}
impl Default for N15_D {
    fn default() -> Self {
        { N15_D { touched: false } }
    }
}
pub fn test_1_5_21() -> () {
    let a: Value<N15_A> = Rc::new(RefCell::new(<N15_A>::default()));
    unimplemented!();
    unimplemented!();
    let mut b: N15_B = <N15_B>::default();
    const _: () = assert!(true, "!has_weak_order(b, b)");;
    let mut c: N15_C = <N15_C>::default();
    const _: () = assert!(true, "!has_weak_order(c, c)");;
    let d1: Value<N15_D> = Rc::new(RefCell::new(<N15_D>::default()));
    let d2: Value<N15_D> = Rc::new(RefCell::new(<N15_D>::default()));
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_1_1_7() });
    ({ test_1_2_11() });
    ({ test_1_3_12() });
    ({ test_1_3_13() });
    ({ test_1_3_14() });
    ({ test_1_4_17() });
    ({ test_1_5_21() });
    const _: () = assert!(true, "test_1_3<float>()");;
    const _: () = assert!(true, "test_1_3<double>()");;
    const _: () = assert!(true, "test_1_3<long double>()");;
    const _: () = assert!(true, "test_1_4()");;
    const _: () = assert!(true, "test_1_5()");;
    return 0_();
}
pub trait N12_WeakOrderImpl {
    fn to_(&self) -> ();
}
impl N12_WeakOrderImpl for Ptr<N12_WeakOrder> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub trait N14_BImpl {
    fn operator_cmp(&self, _a0: Ptr<N14_B>) -> () {
        unimplemented!()
    }
}
impl N14_BImpl for Ptr<N14_B> {}
pub trait N14_CImpl {
    fn operator_eq(&self, _a0: Ptr<N14_C>) -> () {
        unimplemented!()
    }
    fn operator_cmp(&self, rhs: Ptr<N14_C>) -> ();
}
impl N14_CImpl for Ptr<N14_C> {
    fn operator_cmp(&self, rhs: Ptr<N14_C>) -> () {
        rhs.with(|__s| __s.touched.clone()).write(true);
        return unimplemented!();
    }
}
pub trait N15_StrongOrderImpl {
    fn to__1(&self) -> ();
}
impl N15_StrongOrderImpl for Ptr<N15_StrongOrder> {
    fn to__1(&self) -> () {
        return unimplemented!();
    }
}
pub trait N15_WeakOrderImpl {
    fn to_(&self) -> ();
}
impl N15_WeakOrderImpl for Ptr<N15_WeakOrder> {
    fn to_(&self) -> () {
        return unimplemented!();
    }
}
pub fn __cpp2rust_init_globals() {}
