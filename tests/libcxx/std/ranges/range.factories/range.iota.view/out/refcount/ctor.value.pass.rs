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
pub fn operator_add_assign_2(lhs: Ptr<SomeInt>, mut rhs: i32) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_3(lhs: Ptr<SomeInt>, mut rhs: i32) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_4(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_5(mut lhs: SomeInt, mut rhs: SomeInt) -> i32 {
    return (lhs.value_ - rhs.value_);
}
pub fn operator_add_6(mut lhs: SomeInt, mut rhs: i32) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs)) });
}
pub fn operator_sub_7(mut lhs: SomeInt, mut rhs: i32) -> i32 {
    return (lhs.value_ - rhs);
}
pub fn operator_add_8(mut lhs: i32, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs + rhs.value_)) });
}
pub fn operator_sub_9(mut lhs: i32, mut rhs: SomeInt) -> i32 {
    return (lhs - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SomeInt {
    #[offset(0)]
    pub value_: i32,
}
impl SomeInt {
    pub fn new(value: Option<i32>) -> Self {
        let mut value: i32 = value.unwrap_or_else(|| 0);
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
pub fn operator_sub_13(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> i32 {
    return (lhs.value_ - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NotIncrementable {
    #[offset(0)]
    pub value_: i32,
}
impl NotIncrementable {
    pub fn new(value: Option<i32>) -> Self {
        let mut value: i32 = value.unwrap_or_else(|| 0);
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
pub fn operator_sub_17(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> i32 {
    return (lhs.value_ - rhs.value_);
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct NotDecrementable {
    #[offset(0)]
    pub value_: i32,
}
impl NotDecrementable {
    pub fn new(value: Option<i32>) -> Self {
        let mut value: i32 = value.unwrap_or_else(|| 0);
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
pub type CtorKind = u32;
pub const CtorKind_DefaultTo42: CtorKind = 0;
pub const CtorKind_ValueCtor: CtorKind = 1;
pub fn operator_eq_18(lhs: SomeIntComparable, rhs: SomeIntComparable) -> bool {
    let lhs: Value<SomeIntComparable> = Rc::new(RefCell::new(lhs));
    let rhs: Value<SomeIntComparable> = Rc::new(RefCell::new(rhs));
    return ({
        let _arg0: Ptr<SomeInt> = field_ptr!(rhs.as_pointer(), value_);
        SomeIntImpl::operator_eq(&field_ptr!(lhs.as_pointer(), value_), _arg0)
    });
}
pub fn operator_eq_19(lhs: SomeIntComparable, rhs: SomeInt) -> bool {
    let lhs: Value<SomeIntComparable> = Rc::new(RefCell::new(lhs));
    let rhs: Value<SomeInt> = Rc::new(RefCell::new(rhs));
    return ({ SomeIntImpl::operator_eq(&field_ptr!(lhs.as_pointer(), value_), rhs.as_pointer()) });
}
pub fn operator_eq_20(lhs: SomeInt, rhs: SomeIntComparable) -> bool {
    let lhs: Value<SomeInt> = Rc::new(RefCell::new(lhs));
    let rhs: Value<SomeIntComparable> = Rc::new(RefCell::new(rhs));
    return ({ SomeIntImpl::operator_eq(&lhs.as_pointer(), field_ptr!(rhs.as_pointer(), value_)) });
}
pub fn operator_sub_21(mut lhs: SomeIntComparable, mut rhs: SomeIntComparable) -> i32 {
    return ({
        let _lhs: SomeInt = lhs.value_.clone();
        let _rhs: SomeInt = rhs.value_.clone();
        operator_sub_5(_lhs, _rhs)
    });
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct SomeIntComparable {
    #[offset(0)]
    #[byte_size(4)]
    pub value_: SomeInt,
}
impl SomeIntComparable {
    pub fn new() -> Self {
        Self {
            value_: SomeInt::new({ Some(10) }),
        }
    }
}
impl CppEq for SomeIntComparable {
    fn equal(__lhs: &Ptr<Self>, __rhs: &Ptr<Self>) -> bool {
        operator_eq_18(__lhs.read(), __rhs.read())
    }
}
impl Default for SomeIntComparable {
    fn default() -> Self {
        { SomeIntComparable::new() }
    }
}
pub fn test_22() -> bool {
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(({ (unimplemented!()).value_ } == 42));
        assert!(!(unimplemented!()));
        const _: () = assert!(
            true,
            "std::same_as<decltype(io.end()), std::unreachable_sentinel_t>"
        );;
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            ({ (*next_23.with(Value::clone).borrow_mut())(unimplemented!(), (10 as ()),) })
                == unimplemented!()
        );
    }
    {
        const _: () = assert!(
            true,
            "!std::is_convertible_v<std::ranges::iota_view<SomeInt>, SomeInt>"
        );;
        const _: () = assert!(
            true,
            "std::is_constructible_v<std::ranges::iota_view<SomeInt>, SomeInt>"
        );;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_22() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub trait NotDecrementableImpl {
    fn operator_eq(&self, _a0: Ptr<NotDecrementable>) -> bool;
    fn operator_inc_3(&self) -> Ptr<NotDecrementable>;
    fn operator_post_inc_4(&self, _a0: i32);
}
impl NotDecrementableImpl for Ptr<NotDecrementable> {
    fn operator_eq(&self, _a0: Ptr<NotDecrementable>) -> bool {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
    fn operator_inc_3(&self) -> Ptr<NotDecrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: i32) {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
    }
}
pub trait NotIncrementableImpl {
    fn operator_eq(&self, _a0: Ptr<NotIncrementable>) -> bool;
    fn operator_inc_3(&self) -> Ptr<NotIncrementable>;
    fn operator_post_inc_4(&self, _a0: i32);
    fn operator_dec(&self) -> Ptr<NotIncrementable>;
}
impl NotIncrementableImpl for Ptr<NotIncrementable> {
    fn operator_eq(&self, _a0: Ptr<NotIncrementable>) -> bool {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
    fn operator_inc_3(&self) -> Ptr<NotIncrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: i32) {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
    }
    fn operator_dec(&self) -> Ptr<NotIncrementable> {
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
}
pub trait SomeIntImpl {
    fn operator_cmp(&self, _a0: Ptr<SomeInt>) -> std::cmp::Ordering;
    fn operator_inc_3(&self) -> Ptr<SomeInt>;
    fn operator_post_inc_4(&self, _a0: i32) -> SomeInt;
    fn operator_dec_5(&self) -> Ptr<SomeInt>;
    fn operator_post_dec_6(&self, _a0: i32) -> SomeInt;
    fn operator_eq(&self, _a0: Ptr<SomeInt>) -> bool;
}
impl SomeIntImpl for Ptr<SomeInt> {
    fn operator_cmp(&self, _a0: Ptr<SomeInt>) -> std::cmp::Ordering {
        {
            let cmp: Value<std::cmp::Ordering> = Rc::new(RefCell::new(std::cmp::Ord::cmp(
                &((*self).with(|__s| __s.value_)),
                &(_a0.with(|__s| __s.value_)),
            )));
            if !((*cmp.borrow()) == std::cmp::Ordering::Equal) {
                return (*cmp.borrow());
            }
        }
        return std::cmp::Ordering::Equal;
    }
    fn operator_inc_3(&self) -> Ptr<SomeInt> {
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return (*self).clone();
    }
    fn operator_post_inc_4(&self, mut _a0: i32) -> SomeInt {
        let mut tmp: SomeInt = (*(*self).upgrade().deref()).clone();
        field!((*self), value_).with_mut(|__v| __v.prefix_inc());
        return tmp;
    }
    fn operator_dec_5(&self) -> Ptr<SomeInt> {
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return (*self).clone();
    }
    fn operator_post_dec_6(&self, mut _a0: i32) -> SomeInt {
        let mut tmp: SomeInt = (*(*self).upgrade().deref()).clone();
        field!((*self), value_).with_mut(|__v| __v.prefix_dec());
        return tmp;
    }
    fn operator_eq(&self, _a0: Ptr<SomeInt>) -> bool {
        return ({ (*self).with(|__s| __s.value_) } == { _a0.with(|__s| __s.value_) });
    }
}
pub trait SomeIntComparableImpl {
    fn operator_inc_2(&self) -> Ptr<SomeIntComparable>;
    fn operator_post_inc_3(&self, _a0: i32) -> SomeIntComparable;
    fn operator_dec(&self) -> SomeIntComparable;
}
impl SomeIntComparableImpl for Ptr<SomeIntComparable> {
    fn operator_inc_2(&self) -> Ptr<SomeIntComparable> {
        ({ SomeIntImpl::operator_inc_3(&field_ptr!((*self), value_)) });
        return (*self).clone();
    }
    fn operator_post_inc_3(&self, mut _a0: i32) -> SomeIntComparable {
        let mut tmp: SomeIntComparable = (*(*self).upgrade().deref()).clone();
        ({ SomeIntImpl::operator_inc_3(&field_ptr!((*self), value_)) });
        return tmp;
    }
    fn operator_dec(&self) -> SomeIntComparable {
        ({ SomeIntImpl::operator_dec_5(&field_ptr!((*self), value_)) });
        return (*(*self).upgrade().deref()).clone();
    }
}
pub fn __cpp2rust_init_globals() {}
