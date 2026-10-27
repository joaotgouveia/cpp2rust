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
pub struct test_macros_detail_is_same_std_ranges_iota_view_SomeInt__std_unreachable_sentinel_t___std_ranges_iota_view_SomeInt__std_unreachable_sentinel_t__
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_SomeInt__SomeInt___std_ranges_iota_view_SomeInt__SomeInt__
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_SomeInt__IntComparableWith_SomeInt____std_ranges_iota_view_SomeInt__IntComparableWith_SomeInt___
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view___int128__std_unreachable_sentinel_t___std_ranges_iota_view___int128__std_unreachable_sentinel_t__
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view___int128____int128___std_ranges_iota_view___int128____int128__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned___int128__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned___int128__std_unreachable_sentinel_t__
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned___int128__unsigned___int128___std_ranges_iota_view_unsigned___int128__unsigned___int128__
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_long__std_unreachable_sentinel_t___std_ranges_iota_view_long__std_unreachable_sentinel_t__
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_long__IntComparableWith_long____std_ranges_iota_view_long__IntComparableWith_long___
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_long__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned_long__std_unreachable_sentinel_t__
{}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_long__IntComparableWith_unsigned_long____std_ranges_iota_view_unsigned_long__IntComparableWith_unsigned_long___
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_int__std_unreachable_sentinel_t___std_ranges_iota_view_int__std_unreachable_sentinel_t__
{}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_int__IntComparableWith_int____std_ranges_iota_view_int__IntComparableWith_int___
{}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_int__int___std_ranges_iota_view_int__int__
{}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_int__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned_int__std_unreachable_sentinel_t__
{}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_int__unsigned_int___std_ranges_iota_view_unsigned_int__unsigned_int__
{}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_int__IntComparableWith_unsigned_int____std_ranges_iota_view_unsigned_int__IntComparableWith_unsigned_int___
{}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_short__std_unreachable_sentinel_t___std_ranges_iota_view_short__std_unreachable_sentinel_t__
{}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_short__short___std_ranges_iota_view_short__short__
{}
pub type anon_19 = ();
pub const anon_19_value: anon_19 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_short__IntComparableWith_short____std_ranges_iota_view_short__IntComparableWith_short___
{}
pub type anon_20 = ();
pub const anon_20_value: anon_20 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_short__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned_short__std_unreachable_sentinel_t__
{}
pub type anon_21 = ();
pub const anon_21_value: anon_21 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_short__IntComparableWith_unsigned_short____std_ranges_iota_view_unsigned_short__IntComparableWith_unsigned_short___
{}
pub fn operator_add_assign_22(lhs: Ptr<SomeInt>, rhs: Ptr<SomeInt>) -> Ptr<SomeInt> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_23(lhs: Ptr<SomeInt>, rhs: Ptr<SomeInt>) -> Ptr<SomeInt> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_assign_24(lhs: Ptr<SomeInt>, mut rhs: ()) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_25(lhs: Ptr<SomeInt>, mut rhs: ()) -> Ptr<SomeInt> {
    {
        let __rhs = rhs;
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_26(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_27(mut lhs: SomeInt, mut rhs: SomeInt) -> () {
    return (lhs.value_ - rhs.value_);
}
pub fn operator_add_28(mut lhs: SomeInt, mut rhs: ()) -> SomeInt {
    return SomeInt::new({ Some((lhs.value_ + rhs)) });
}
pub fn operator_sub_29(mut lhs: SomeInt, mut rhs: ()) -> () {
    return (lhs.value_ - rhs);
}
pub fn operator_add_30(mut lhs: (), mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some((lhs + rhs.value_)) });
}
pub fn operator_sub_31(mut lhs: (), mut rhs: SomeInt) -> () {
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
impl std::cmp::Ord for SomeInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            SomeIntImpl::operator_cmp(
                &Rc::new(RefCell::new(SomeInt {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(SomeInt {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::PartialOrd for SomeInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for SomeInt {
    fn eq(&self, other: &Self) -> bool {
        {
            SomeIntImpl::operator_cmp(
                &Rc::new(RefCell::new(SomeInt {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(SomeInt {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for SomeInt {}
impl Default for SomeInt {
    fn default() -> Self {
        { SomeInt::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct IntComparableWith_SomeInt_ {
    #[offset(0)]
    #[byte_size(4)]
    pub value_: SomeInt,
}
impl IntComparableWith_SomeInt_ {
    pub fn new(value: Option<SomeInt>) -> Self {
        let mut value: SomeInt = value.unwrap_or_else(|| SomeInt::new(None));
        Self {
            value_: (value).clone(),
        }
    }
}
impl Default for IntComparableWith_SomeInt_ {
    fn default() -> Self {
        { IntComparableWith_SomeInt_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct IntComparableWith_long_ {
    #[offset(0)]
    pub value_: (),
}
impl IntComparableWith_long_ {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unimplemented!());
        Self { value_: value }
    }
}
impl Default for IntComparableWith_long_ {
    fn default() -> Self {
        { IntComparableWith_long_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct IntComparableWith_unsigned_long_ {
    #[offset(0)]
    pub value_: (),
}
impl IntComparableWith_unsigned_long_ {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unimplemented!());
        Self { value_: value }
    }
}
impl Default for IntComparableWith_unsigned_long_ {
    fn default() -> Self {
        { IntComparableWith_unsigned_long_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct IntComparableWith_int_ {
    #[offset(0)]
    pub value_: (),
}
impl IntComparableWith_int_ {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unimplemented!());
        Self { value_: value }
    }
}
impl Default for IntComparableWith_int_ {
    fn default() -> Self {
        { IntComparableWith_int_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct IntComparableWith_unsigned_int_ {
    #[offset(0)]
    pub value_: (),
}
impl IntComparableWith_unsigned_int_ {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unimplemented!());
        Self { value_: value }
    }
}
impl Default for IntComparableWith_unsigned_int_ {
    fn default() -> Self {
        { IntComparableWith_unsigned_int_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct IntComparableWith_short_ {
    #[offset(0)]
    pub value_: (),
}
impl IntComparableWith_short_ {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unimplemented!());
        Self { value_: value }
    }
}
impl Default for IntComparableWith_short_ {
    fn default() -> Self {
        { IntComparableWith_short_::new(None) }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(2)]
pub struct IntComparableWith_unsigned_short_ {
    #[offset(0)]
    pub value_: (),
}
impl IntComparableWith_unsigned_short_ {
    pub fn new(value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unimplemented!());
        Self { value_: value }
    }
}
impl Default for IntComparableWith_unsigned_short_ {
    fn default() -> Self {
        { IntComparableWith_unsigned_short_::new(None) }
    }
}
pub fn operator_add_assign_32(
    lhs: Ptr<NotIncrementable>,
    rhs: Ptr<NotIncrementable>,
) -> Ptr<NotIncrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_33(
    lhs: Ptr<NotIncrementable>,
    rhs: Ptr<NotIncrementable>,
) -> Ptr<NotIncrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_34(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> NotIncrementable {
    return NotIncrementable::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_35(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> () {
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
impl std::cmp::PartialEq for NotIncrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            NotIncrementableImpl::operator_eq(
                &Rc::new(RefCell::new(NotIncrementable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NotIncrementable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NotIncrementable {}
impl Default for NotIncrementable {
    fn default() -> Self {
        { NotIncrementable::new(None) }
    }
}
const _: () = assert!(!(false), "!std::incrementable<NotIncrementable>");
pub fn operator_add_assign_36(
    lhs: Ptr<NotDecrementable>,
    rhs: Ptr<NotDecrementable>,
) -> Ptr<NotDecrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v + __rhs)
    };
    return (lhs).clone();
}
pub fn operator_sub_assign_37(
    lhs: Ptr<NotDecrementable>,
    rhs: Ptr<NotDecrementable>,
) -> Ptr<NotDecrementable> {
    {
        let __rhs = { rhs.with(|__s| __s.value_) };
        field!(lhs, value_).with_mut(|__v| *__v = *__v - __rhs)
    };
    return (lhs).clone();
}
pub fn operator_add_38(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> NotDecrementable {
    return NotDecrementable::new({ Some((lhs.value_ + rhs.value_)) });
}
pub fn operator_sub_39(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> () {
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
impl std::cmp::PartialEq for NotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        {
            NotDecrementableImpl::operator_eq(
                &Rc::new(RefCell::new(NotDecrementable {
                    value_: self.value_.clone(),
                }))
                .as_pointer(),
                Rc::new(RefCell::new(NotDecrementable {
                    value_: other.value_.clone(),
                }))
                .as_pointer(),
            )
        }
    }
}
impl std::cmp::Eq for NotDecrementable {}
impl Default for NotDecrementable {
    fn default() -> Self {
        { NotDecrementable::new(None) }
    }
}
pub type CtorKind = ();
pub const CtorKind_DefaultTo42: CtorKind = 0;
pub const CtorKind_ValueCtor: CtorKind = 1;
pub fn testType_40(u: SomeInt) {
    let u: Value<SomeInt> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<SomeInt> = Rc::new(RefCell::new(SomeInt::new({ Some(10_()) })));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<SomeInt> = Rc::new(RefCell::new(SomeInt::new({ Some(10_()) })));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_42(u: IntComparableWith_SomeInt_) {
    let u: Value<IntComparableWith_SomeInt_> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<SomeInt> = Rc::new(RefCell::new(SomeInt::new({ Some(10_()) })));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<SomeInt> = Rc::new(RefCell::new(SomeInt::new({ Some(10_()) })));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_43(u: ()) {
    let u: Value<()> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_44(u: ()) {
    let u: Value<()> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_45(u: IntComparableWith_long_) {
    let u: Value<IntComparableWith_long_> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_46(u: IntComparableWith_unsigned_long_) {
    let u: Value<IntComparableWith_unsigned_long_> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_47(u: IntComparableWith_int_) {
    let u: Value<IntComparableWith_int_> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_48(u: ()) {
    let u: Value<()> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_49(u: ()) {
    let u: Value<()> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_50(u: IntComparableWith_unsigned_int_) {
    let u: Value<IntComparableWith_unsigned_int_> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_51(u: ()) {
    let u: Value<()> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_52(u: IntComparableWith_short_) {
    let u: Value<IntComparableWith_short_> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub fn testType_53(u: IntComparableWith_unsigned_short_) {
    let u: Value<IntComparableWith_unsigned_short_> = Rc::new(RefCell::new(u));
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer())
            })
            .clone(),
        ));
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: Value<()> = Rc::new(RefCell::new(
            ({
                let ___start: Value<()> = Rc::new(RefCell::new(10_()));
                (*iota_41.with(Value::clone).borrow_mut())(___start.as_pointer(), u.as_pointer())
            }),
        ));
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct X {}
pub fn test_54() -> () {
    ({ testType_40(SomeInt::new({ Some(10_()) })) });
    ({
        testType_42(IntComparableWith_SomeInt_::new({
            Some(SomeInt::new({ Some(10_()) }))
        }))
    });
    ({ testType_43(10_()) });
    ({ testType_44(10_()) });
    ({ testType_45(IntComparableWith_long_::new({ Some(10_()) })) });
    ({ testType_46(IntComparableWith_unsigned_long_::new({ Some(10_()) })) });
    ({ testType_47(IntComparableWith_int_::new({ Some(10_()) })) });
    ({ testType_48(10_()) });
    ({ testType_49(10_()) });
    ({ testType_50(IntComparableWith_unsigned_int_::new({ Some(10_()) })) });
    ({ testType_51(10_()) });
    ({ testType_52(IntComparableWith_short_::new({ Some(10_()) })) });
    ({ testType_53(IntComparableWith_unsigned_short_::new({ Some(10_()) })) });
    {
        const _: () = assert!(true, "std::is_invocable_v<decltype(std::views::iota), int>");;
        const _: () = assert!(true, "!std::is_invocable_v<decltype(std::views::iota), X>");;
        const _: () = assert!(
            true,
            "std::is_invocable_v<decltype(std::views::iota), int, int>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::views::iota), int, X>"
        );;
    }
    {
        const _: () = assert!(
            true,
            "std::same_as<decltype(std::views::iota), decltype(std::ranges::views::iota)>"
        );;
    }
    {
        const _: () = assert!(!(false), "!CanDoubleWrap<int>");;
        const _: () = assert!(!(false), "!CanDoubleWrap<SomeInt>");;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::views::iota), decltype(std::views::iota(0))>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::views::iota), decltype(std::views::iota(82))>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::views::iota), decltype(std::views::iota(SomeInt(0)))>"
        );;
        const _: () = assert!(
            true,
            "!std::is_invocable_v<decltype(std::views::iota), decltype(std::views::iota(SomeInt(94)))>"
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_54() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait IntComparableWith_SomeInt_Impl {
    fn operator_inc_2(&self) -> Ptr<IntComparableWith_SomeInt_> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) -> IntComparableWith_SomeInt_ {
        unimplemented!()
    }
    fn operator_dec(&self) -> IntComparableWith_SomeInt_ {
        unimplemented!()
    }
}
impl IntComparableWith_SomeInt_Impl for Ptr<IntComparableWith_SomeInt_> {}
pub trait IntComparableWith_int_Impl {
    fn operator_inc_2(&self) -> Ptr<IntComparableWith_int_> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) -> IntComparableWith_int_ {
        unimplemented!()
    }
    fn operator_dec(&self) -> IntComparableWith_int_ {
        unimplemented!()
    }
}
impl IntComparableWith_int_Impl for Ptr<IntComparableWith_int_> {}
pub trait IntComparableWith_long_Impl {
    fn operator_inc_2(&self) -> Ptr<IntComparableWith_long_> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) -> IntComparableWith_long_ {
        unimplemented!()
    }
    fn operator_dec(&self) -> IntComparableWith_long_ {
        unimplemented!()
    }
}
impl IntComparableWith_long_Impl for Ptr<IntComparableWith_long_> {}
pub trait IntComparableWith_short_Impl {
    fn operator_inc_2(&self) -> Ptr<IntComparableWith_short_> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) -> IntComparableWith_short_ {
        unimplemented!()
    }
    fn operator_dec(&self) -> IntComparableWith_short_ {
        unimplemented!()
    }
}
impl IntComparableWith_short_Impl for Ptr<IntComparableWith_short_> {}
pub trait IntComparableWith_unsigned_int_Impl {
    fn operator_inc_2(&self) -> Ptr<IntComparableWith_unsigned_int_> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) -> IntComparableWith_unsigned_int_ {
        unimplemented!()
    }
    fn operator_dec(&self) -> IntComparableWith_unsigned_int_ {
        unimplemented!()
    }
}
impl IntComparableWith_unsigned_int_Impl for Ptr<IntComparableWith_unsigned_int_> {}
pub trait IntComparableWith_unsigned_long_Impl {
    fn operator_inc_2(&self) -> Ptr<IntComparableWith_unsigned_long_> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) -> IntComparableWith_unsigned_long_ {
        unimplemented!()
    }
    fn operator_dec(&self) -> IntComparableWith_unsigned_long_ {
        unimplemented!()
    }
}
impl IntComparableWith_unsigned_long_Impl for Ptr<IntComparableWith_unsigned_long_> {}
pub trait IntComparableWith_unsigned_short_Impl {
    fn operator_inc_2(&self) -> Ptr<IntComparableWith_unsigned_short_> {
        unimplemented!()
    }
    fn operator_post_inc_3(&self, mut _a0: ()) -> IntComparableWith_unsigned_short_ {
        unimplemented!()
    }
    fn operator_dec(&self) -> IntComparableWith_unsigned_short_ {
        unimplemented!()
    }
}
impl IntComparableWith_unsigned_short_Impl for Ptr<IntComparableWith_unsigned_short_> {}
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
