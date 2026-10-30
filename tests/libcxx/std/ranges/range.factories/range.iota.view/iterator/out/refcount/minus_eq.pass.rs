extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn operator_add_assign_0(lhs: Ptr<SomeInt>, rhs: Ptr<SomeInt>) -> Ptr<SomeInt> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_1(lhs: Ptr<SomeInt>, rhs: Ptr<SomeInt>) -> Ptr<SomeInt> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_assign_2(lhs: Ptr<SomeInt>, mut rhs: ()) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_3(lhs: Ptr<SomeInt>, mut rhs: ()) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_4(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_5(mut lhs: SomeInt, mut rhs: SomeInt) -> () {
    return (lhs.value_ - rhs.value_);
}
pub fn operator_add_6(mut lhs: SomeInt, mut rhs: ()) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs)) });
}
pub fn operator_sub_7(mut lhs: SomeInt, mut rhs: ()) -> () {
    return (lhs.value_ - rhs);
}
pub fn operator_add_8(mut lhs: (), mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs + rhs.value_)) });
}
pub fn operator_sub_9(mut lhs: (), mut rhs: SomeInt) -> () {
    return (lhs - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SomeInt {
    #[offset(0)]
    pub value_: (),
}
impl SomeInt {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| 0_());
        Self { value_: value }
    }
}
impl CppLess for SomeInt {
    fn less(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SomeIntImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Less
    }
}
impl CppEq for SomeInt {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        SomeIntImpl::operator_cmp(__lhs, __rhs.clone()) == std::cmp::Ordering::Equal
    }
}
impl Default for SomeInt {
    fn default() -> Self {
        { SomeInt::new(None) }
    }
}
pub fn operator_add_assign_10(
    lhs: Ptr<NotIncrementable>,
    rhs: Ptr<NotIncrementable>,
) -> Ptr<NotIncrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_11(
    lhs: Ptr<NotIncrementable>,
    rhs: Ptr<NotIncrementable>,
) -> Ptr<NotIncrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_12(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> NotIncrementable {
    return NotIncrementable::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_13(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> () {
    return (lhs.value_ - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NotIncrementable {
    #[offset(0)]
    pub value_: (),
}
impl NotIncrementable {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| 0_());
        Self { value_: value }
    }
}
impl CppEq for NotIncrementable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        NotIncrementableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for NotIncrementable {
    fn default() -> Self {
        { NotIncrementable::new(None) }
    }
}
const _: () = assert!(!(false), "!std::incrementable<NotIncrementable>");
pub fn operator_add_assign_14(
    lhs: Ptr<NotDecrementable>,
    rhs: Ptr<NotDecrementable>,
) -> Ptr<NotDecrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_15(
    lhs: Ptr<NotDecrementable>,
    rhs: Ptr<NotDecrementable>,
) -> Ptr<NotDecrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_16(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> NotDecrementable {
    return NotDecrementable::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_17(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> () {
    return (lhs.value_ - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NotDecrementable {
    #[offset(0)]
    pub value_: (),
}
impl NotDecrementable {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| 0_());
        Self { value_: value }
    }
}
impl CppEq for NotDecrementable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        NotDecrementableImpl::operator_eq(__lhs, __rhs.clone())
    }
}
impl Default for NotDecrementable {
    fn default() -> Self {
        { NotDecrementable::new(None) }
    }
}
pub type CtorKind = ();
pub const CtorKind_DefaultTo42: CtorKind = 0;
pub const CtorKind_ValueCtor: CtorKind = 1;
pub fn test_18() -> () {
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "std::is_reference_v<decltype(iter2 -= 5)>");;
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "std::is_reference_v<decltype(iter2 -= 5)>");;
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "std::is_reference_v<decltype(iter2 -= 5)>");;
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter1: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let iter2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "std::is_reference_v<decltype(iter2 -= -5)>");;
    }
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
    ({ test_18() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait NotDecrementableImpl {
    fn operator_eq(&self, _a0: Ptr<NotDecrementable>) -> ();
    fn operator_inc_3(&self) -> Ptr<NotDecrementable>;
    fn operator_post_inc_4(&self, _a0: ());
}
impl NotDecrementableImpl for Ptr<NotDecrementable> {
    fn operator_eq(&self, _a0: Ptr<NotDecrementable>) -> () {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
    fn operator_inc_3(&self) -> Ptr<NotDecrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait NotIncrementableImpl {
    fn operator_eq(&self, _a0: Ptr<NotIncrementable>) -> ();
    fn operator_inc_3(&self) -> Ptr<NotIncrementable>;
    fn operator_post_inc_4(&self, _a0: ());
    fn operator_dec(&self) -> Ptr<NotIncrementable>;
}
impl NotIncrementableImpl for Ptr<NotIncrementable> {
    fn operator_eq(&self, _a0: Ptr<NotIncrementable>) -> () {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
    fn operator_inc_3(&self) -> Ptr<NotIncrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: ()) {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
    }
    fn operator_dec(&self) -> Ptr<NotIncrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
}
pub trait SomeIntImpl {
    fn operator_cmp(&self, _a0: Ptr<SomeInt>) -> ();
    fn operator_inc_3(&self) -> Ptr<SomeInt>;
    fn operator_post_inc_4(&self, _a0: ()) -> SomeInt;
    fn operator_dec_5(&self) -> Ptr<SomeInt>;
    fn operator_post_dec_6(&self, _a0: ()) -> SomeInt;
    fn operator_eq(&self, _a0: Ptr<SomeInt>) -> ();
}
impl SomeIntImpl for Ptr<SomeInt> {
    fn operator_cmp(&self, _a0: Ptr<SomeInt>) -> () {
        {
            let cmp: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.value_)),
                &(_a0.with(|__s| __s.value_)),
            )));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    fn operator_inc_3(&self) -> Ptr<SomeInt> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: ()) -> SomeInt {
        let mut tmp: SomeInt = (*(*self).upgrade().deref()).clone();
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return tmp;
    }
    fn operator_dec_5(&self) -> Ptr<SomeInt> {
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_6(&self, mut _a0: ()) -> SomeInt {
        let mut tmp: SomeInt = (*(*self).upgrade().deref()).clone();
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return tmp;
    }
    fn operator_eq(&self, _a0: Ptr<SomeInt>) -> () {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
}
pub fn __cpp2rust_init_globals() {}
