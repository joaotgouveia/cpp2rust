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
pub struct IntType {
    #[offset(0)]
    pub val: (),
}
impl IntType {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
impl std::cmp::PartialEq for IntType {
    fn eq(&self, other: &Self) -> bool {
        {
            IntTypeImpl::operator_eq(
                &Rc::new(RefCell::new(IntType {
                    val: self.val.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(IntType {
                    val: other.val.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for IntType {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct IntTypeNC {
    #[offset(0)]
    pub val: (),
}
impl IntTypeNC {
    pub fn new_1(mut v: ()) -> Self {
        Self { val: v }
    }
}
pub fn check_operator_constraints_0(m: (), mut idxs: ()) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_1(m: (), mut idxs: IntType) -> () {
    let m: Value<()> = Rc::new(RefCell::new(m));
    &(unimplemented!());
    return true;
}
pub fn check_operator_constraints_2(_a0: (), mut _a1: (), mut _a2: ()) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_3(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn check_operator_constraints_4(_a0: (), mut _a1: IntType) -> () {
    let _a0: Value<()> = Rc::new(RefCell::new(_a0));
    return false;
}
pub fn iterate_left_5(m: (), count: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        count.with_mut(|__v| __v.postfix_inc());
    }
}
pub fn iterate_left_6(m: (), count: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args: () = i;
                iterate_left_7(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_7(m: (), count: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        count.with_mut(|__v| __v.postfix_inc());
    }
}
pub fn iterate_left_8(m: (), count: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args: () = i;
                iterate_left_9(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_9(m: (), count: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        count.with_mut(|__v| __v.postfix_inc());
    }
}
pub fn iterate_left_10(m: (), count: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args: () = i;
                iterate_left_11(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_11(m: (), count: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_12(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_12(m: (), count: Ptr<()>, mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        count.with_mut(|__v| __v.postfix_inc());
    }
}
pub fn iterate_left_13(m: (), count: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args: () = i;
                iterate_left_14(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_14(m: (), count: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_15(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_15(m: (), count: Ptr<()>, mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_left_16(_m, _count, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_16(m: (), count: Ptr<()>, mut args_2: (), mut args_3: (), mut args_4: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_left_17(_m, _count, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_17(
    m: (),
    count: Ptr<()>,
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
    mut args_5: (),
) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        count.with_mut(|__v| __v.postfix_inc());
    }
}
pub fn iterate_left_18(m: (), count: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args: () = i;
                iterate_left_19(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_19(m: (), count: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_20(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_20(m: (), count: Ptr<()>, mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_left_21(_m, _count, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_21(m: (), count: Ptr<()>, mut args_2: (), mut args_3: (), mut args_4: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_left_22(_m, _count, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_22(
    m: (),
    count: Ptr<()>,
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
    mut args_5: (),
) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        count.with_mut(|__v| __v.postfix_inc());
    }
}
pub fn iterate_left_23(m: (), count: Ptr<()>) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 0);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args: () = i;
                iterate_left_24(_m, _count, _args)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_24(m: (), count: Ptr<()>, mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 1);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args;
                iterate_left_25(_m, _count, _args_2, _args_3)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_25(m: (), count: Ptr<()>, mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 2);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                iterate_left_26(_m, _count, _args_2, _args_3, _args_4)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_26(m: (), count: Ptr<()>, mut args_2: (), mut args_3: (), mut args_4: ()) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 3);
    {
        let mut i: () = 0_();
        'loop_: while (i < unimplemented!()) {
            ({
                let _m: () = unimplemented!();
                let _count: Ptr<()> = (count).clone();
                let _args_2: () = i;
                let _args_3: () = args_2;
                let _args_4: () = args_3;
                let _args_5: () = args_4;
                iterate_left_27(_m, _count, _args_2, _args_3, _args_4, _args_5)
            });
            i.postfix_inc();
        }
    }
}
pub fn iterate_left_27(
    m: (),
    count: Ptr<()>,
    mut args_2: (),
    mut args_3: (),
    mut args_4: (),
    mut args_5: (),
) {
    let m: Value<()> = Rc::new(RefCell::new(m));
    let mut r: () = ((unimplemented!() - 1_()) - 4);
    {
        const _: () = assert!(true, "");;
        unimplemented!();
        count.with_mut(|__v| __v.postfix_inc());
    }
}
pub fn test_iteration_28() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    ({ iterate_left_5(unimplemented!(), count.as_pointer()) });
}
pub fn test_iteration_29(mut args: ()) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    ({ iterate_left_6(unimplemented!(), count.as_pointer()) });
}
pub fn test_iteration_30() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    ({ iterate_left_8(unimplemented!(), count.as_pointer()) });
}
pub fn test_iteration_31() {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    ({ iterate_left_10(unimplemented!(), count.as_pointer()) });
}
pub fn test_iteration_32(mut args_0: (), mut args_1: (), mut args_2: (), mut args_3: ()) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    ({ iterate_left_13(unimplemented!(), count.as_pointer()) });
}
pub fn test_iteration_33(mut args_0: (), mut args_1: (), mut args_2: ()) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    ({ iterate_left_18(unimplemented!(), count.as_pointer()) });
}
pub fn test_iteration_34(mut args_0: (), mut args_1: ()) {
    let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let count: Value<()> = Rc::new(RefCell::new(0_()));
    ({ iterate_left_23(unimplemented!(), count.as_pointer()) });
}
pub fn test_35() -> () {
    let mut D: () = dynamic_extent_36.with(|rc| *rc.borrow());
    ({ test_iteration_28() });
    ({ test_iteration_29(1_()) });
    ({ test_iteration_29(7_()) });
    ({ test_iteration_30() });
    ({ test_iteration_31() });
    ({ test_iteration_32(1_(), 1_(), 1_(), 1_()) });
    const _: () = assert!(
        true,
        "check_operator_constraints(std::layout_left::mapping<std::extents<int, D>>(std::extents<int, D>(1)), 0)"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(std::layout_left::mapping<std::extents<int, D>>(std::extents<int, D>(1)), 0, 0)"
    );;
    const _: () = assert!(
        true,
        "check_operator_constraints(std::layout_left::mapping<std::extents<int, D>>(std::extents<int, D>(1)), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::layout_left::mapping<std::extents<unsigned, D>>(std::extents<unsigned, D>(1)), IntType(0))"
    );;
    const _: () = assert!(
        true,
        "!check_operator_constraints(\n      std::layout_left::mapping<std::extents<unsigned char, D>>(std::extents<unsigned char, D>(1)), IntType(0))"
    );;
    return true;
}
pub fn test_large_37() -> () {
    let mut D: () = dynamic_extent_36.with(|rc| *rc.borrow());
    ({ test_iteration_33(7_(), 9_(), 10_()) });
    ({ test_iteration_34(7_(), 10_()) });
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
    ({ test_35() });
    const _: () = assert!(true, "test()");;
    ({ test_large_37() });
    return 0_();
}
pub trait IntTypeImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeImpl for Ptr<IntType> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub trait IntTypeNCImpl {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> ();
    fn to__4(&self) -> ();
    fn to__5(&self) -> ();
    fn to__6(&self) -> ();
}
impl IntTypeNCImpl for Ptr<IntTypeNC> {
    fn operator_eq(&self, rhs: Ptr<IntType>) -> () {
        return ({ (*self).with(|__s| __s.val) } == { rhs.with(|__s| __s.val) });
    }
    fn to__4(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__5(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
    fn to__6(&self) -> () {
        return (*self).with(|__s| __s.val);
    }
}
pub fn __cpp2rust_init_globals() {}
