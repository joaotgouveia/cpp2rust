extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_SomeInt__std_unreachable_sentinel_t___std_ranges_iota_view_SomeInt__std_unreachable_sentinel_t__
{}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_SomeInt__SomeInt___std_ranges_iota_view_SomeInt__SomeInt__
{}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_SomeInt__IntComparableWith_SomeInt____std_ranges_iota_view_SomeInt__IntComparableWith_SomeInt___
{}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view___int128__std_unreachable_sentinel_t___std_ranges_iota_view___int128__std_unreachable_sentinel_t__
{}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view___int128____int128___std_ranges_iota_view___int128____int128__
{}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned___int128__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned___int128__std_unreachable_sentinel_t__
{}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned___int128__unsigned___int128___std_ranges_iota_view_unsigned___int128__unsigned___int128__
{}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_long__std_unreachable_sentinel_t___std_ranges_iota_view_long__std_unreachable_sentinel_t__
{}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_long__IntComparableWith_long____std_ranges_iota_view_long__IntComparableWith_long___
{}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_long__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned_long__std_unreachable_sentinel_t__
{}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_long__IntComparableWith_unsigned_long____std_ranges_iota_view_unsigned_long__IntComparableWith_unsigned_long___
{}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_int__std_unreachable_sentinel_t___std_ranges_iota_view_int__std_unreachable_sentinel_t__
{}
pub type anon_12 = ();
pub const anon_12_value: anon_12 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_int__IntComparableWith_int____std_ranges_iota_view_int__IntComparableWith_int___
{}
pub type anon_13 = ();
pub const anon_13_value: anon_13 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_int__int___std_ranges_iota_view_int__int__
{}
pub type anon_14 = ();
pub const anon_14_value: anon_14 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_int__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned_int__std_unreachable_sentinel_t__
{}
pub type anon_15 = ();
pub const anon_15_value: anon_15 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_int__unsigned_int___std_ranges_iota_view_unsigned_int__unsigned_int__
{}
pub type anon_16 = ();
pub const anon_16_value: anon_16 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_int__IntComparableWith_unsigned_int____std_ranges_iota_view_unsigned_int__IntComparableWith_unsigned_int___
{}
pub type anon_17 = ();
pub const anon_17_value: anon_17 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_short__std_unreachable_sentinel_t___std_ranges_iota_view_short__std_unreachable_sentinel_t__
{}
pub type anon_18 = ();
pub const anon_18_value: anon_18 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_short__short___std_ranges_iota_view_short__short__
{}
pub type anon_19 = ();
pub const anon_19_value: anon_19 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_short__IntComparableWith_short____std_ranges_iota_view_short__IntComparableWith_short___
{}
pub type anon_20 = ();
pub const anon_20_value: anon_20 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_short__std_unreachable_sentinel_t___std_ranges_iota_view_unsigned_short__std_unreachable_sentinel_t__
{}
pub type anon_21 = ();
pub const anon_21_value: anon_21 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_ranges_iota_view_unsigned_short__IntComparableWith_unsigned_short____std_ranges_iota_view_unsigned_short__IntComparableWith_unsigned_short___
{}
pub unsafe fn operator_add_assign_22(lhs: *mut SomeInt, rhs: *const SomeInt) -> *mut SomeInt {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_23(lhs: *mut SomeInt, rhs: *const SomeInt) -> *mut SomeInt {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_assign_24(lhs: *mut SomeInt, mut rhs: ()) -> *mut SomeInt {
    (*lhs).value_ += rhs;
    return lhs;
}
pub unsafe fn operator_sub_assign_25(lhs: *mut SomeInt, mut rhs: ()) -> *mut SomeInt {
    (*lhs).value_ -= rhs;
    return lhs;
}
pub unsafe fn operator_add_26(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_27(mut lhs: SomeInt, mut rhs: SomeInt) -> () {
    return ((lhs.value_) - (rhs.value_));
}
pub unsafe fn operator_add_28(mut lhs: SomeInt, mut rhs: ()) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs))) });
}
pub unsafe fn operator_sub_29(mut lhs: SomeInt, mut rhs: ()) -> () {
    return ((lhs.value_) - (rhs));
}
pub unsafe fn operator_add_30(mut lhs: (), mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs) + (rhs.value_))) });
}
pub unsafe fn operator_sub_31(mut lhs: (), mut rhs: SomeInt) -> () {
    return ((lhs) - (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SomeInt {
    pub value_: (),
}
impl SomeInt {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn operator_cmp(&self, _a0: *const SomeInt) -> () {
        {
            let mut cmp: () =
                std::cmp::Ord::cmp(&((*(self as *const SomeInt)).value_), &((*_a0).value_));
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_inc_3(&mut self) -> *mut SomeInt {
        self.value_.prefix_inc();
        return &mut (*(self as *mut SomeInt));
    }
    pub unsafe fn operator_post_inc_4(&mut self, mut _a0: ()) -> SomeInt {
        let mut tmp: SomeInt = (*(self as *mut SomeInt));
        self.value_.prefix_inc();
        return tmp;
    }
    pub unsafe fn operator_dec_5(&mut self) -> *mut SomeInt {
        self.value_.prefix_dec();
        return &mut (*(self as *mut SomeInt));
    }
    pub unsafe fn operator_post_dec_6(&mut self, mut _a0: ()) -> SomeInt {
        let mut tmp: SomeInt = (*(self as *mut SomeInt));
        self.value_.prefix_dec();
        return tmp;
    }
    pub unsafe fn operator_eq(&self, _a0: *const SomeInt) -> () {
        return (((*(self as *const SomeInt)).value_) == ((*_a0).value_));
    }
}
impl std::cmp::Ord for SomeInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { SomeInt::operator_cmp(self, other as *const SomeInt) }
    }
}
impl std::cmp::PartialOrd for SomeInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for SomeInt {
    fn eq(&self, other: &Self) -> bool {
        unsafe { SomeInt::operator_cmp(self, other as *const SomeInt) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for SomeInt {}
impl Default for SomeInt {
    fn default() -> Self {
        unsafe { SomeInt::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_SomeInt_ {
    pub value_: SomeInt,
}
impl IntComparableWith_SomeInt_ {
    pub unsafe fn new(mut value: Option<SomeInt>) -> Self {
        let mut value: SomeInt = value.unwrap_or_else(|| unsafe { SomeInt::new(None) });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_SomeInt_ {
    fn default() -> Self {
        unsafe { IntComparableWith_SomeInt_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_long_ {
    pub value_: (),
}
impl IntComparableWith_long_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_long_ {
    fn default() -> Self {
        unsafe { IntComparableWith_long_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_unsigned_long_ {
    pub value_: (),
}
impl IntComparableWith_unsigned_long_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_unsigned_long_ {
    fn default() -> Self {
        unsafe { IntComparableWith_unsigned_long_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_int_ {
    pub value_: (),
}
impl IntComparableWith_int_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_int_ {
    fn default() -> Self {
        unsafe { IntComparableWith_int_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_unsigned_int_ {
    pub value_: (),
}
impl IntComparableWith_unsigned_int_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_unsigned_int_ {
    fn default() -> Self {
        unsafe { IntComparableWith_unsigned_int_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_short_ {
    pub value_: (),
}
impl IntComparableWith_short_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_short_ {
    fn default() -> Self {
        unsafe { IntComparableWith_short_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_unsigned_short_ {
    pub value_: (),
}
impl IntComparableWith_unsigned_short_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_unsigned_short_ {
    fn default() -> Self {
        unsafe { IntComparableWith_unsigned_short_::new(None) }
    }
}
pub unsafe fn operator_add_assign_32(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_33(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_34(
    mut lhs: NotIncrementable,
    mut rhs: NotIncrementable,
) -> NotIncrementable {
    return NotIncrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_35(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> () {
    return ((lhs.value_) - (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NotIncrementable {
    pub value_: (),
}
impl NotIncrementable {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn operator_eq(&self, _a0: *const NotIncrementable) -> () {
        return (((*(self as *const NotIncrementable)).value_) == ((*_a0).value_));
    }
    pub unsafe fn operator_inc_3(&mut self) -> *mut NotIncrementable {
        self.value_.prefix_inc();
        return &mut (*(self as *mut NotIncrementable));
    }
    pub unsafe fn operator_post_inc_4(&mut self, mut _a0: ()) {
        self.value_.prefix_inc();
    }
    pub unsafe fn operator_dec(&mut self) -> *mut NotIncrementable {
        self.value_.prefix_dec();
        return &mut (*(self as *mut NotIncrementable));
    }
}
impl std::cmp::PartialEq for NotIncrementable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { NotIncrementable::operator_eq(self, other as *const NotIncrementable) }
    }
}
impl std::cmp::Eq for NotIncrementable {}
impl Default for NotIncrementable {
    fn default() -> Self {
        unsafe { NotIncrementable::new(None) }
    }
}
const _: () = assert!(!(false), "!std::incrementable<NotIncrementable>");
pub unsafe fn operator_add_assign_36(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_37(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_38(
    mut lhs: NotDecrementable,
    mut rhs: NotDecrementable,
) -> NotDecrementable {
    return NotDecrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_39(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> () {
    return ((lhs.value_) - (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NotDecrementable {
    pub value_: (),
}
impl NotDecrementable {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn operator_eq(&self, _a0: *const NotDecrementable) -> () {
        return (((*(self as *const NotDecrementable)).value_) == ((*_a0).value_));
    }
    pub unsafe fn operator_inc_3(&mut self) -> *mut NotDecrementable {
        self.value_.prefix_inc();
        return &mut (*(self as *mut NotDecrementable));
    }
    pub unsafe fn operator_post_inc_4(&mut self, mut _a0: ()) {
        self.value_.prefix_inc();
    }
}
impl std::cmp::PartialEq for NotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { NotDecrementable::operator_eq(self, other as *const NotDecrementable) }
    }
}
impl std::cmp::Eq for NotDecrementable {}
impl Default for NotDecrementable {
    fn default() -> Self {
        unsafe { NotDecrementable::new(None) }
    }
}
pub type CtorKind = ();
pub const CtorKind_DefaultTo42: CtorKind = 0;
pub const CtorKind_ValueCtor: CtorKind = 1;
pub unsafe fn testType_40(mut u: SomeInt) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: SomeInt = SomeInt::new({ Some(10_()) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: SomeInt = SomeInt::new({ Some(10_()) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_42(mut u: IntComparableWith_SomeInt_) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: SomeInt = SomeInt::new({ Some(10_()) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: SomeInt = SomeInt::new({ Some(10_()) });
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_43(mut u: ()) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_44(mut u: ()) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_45(mut u: IntComparableWith_long_) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_46(mut u: IntComparableWith_unsigned_long_) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_47(mut u: IntComparableWith_int_) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_48(mut u: ()) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_49(mut u: ()) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_50(mut u: IntComparableWith_unsigned_int_) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_51(mut u: ()) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_52(mut u: IntComparableWith_short_) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn testType_53(mut u: IntComparableWith_unsigned_short_) {
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start)
        });
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        let io: () = (unsafe {
            let mut ___start: () = 10_();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut iota_41))(&mut ___start, &mut u)
        });
        unimplemented!();
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct X {}
pub unsafe fn test_54() -> () {
    (unsafe { testType_40(SomeInt::new({ Some(10_()) })) });
    (unsafe {
        testType_42(IntComparableWith_SomeInt_::new({
            Some(SomeInt::new({ Some(10_()) }))
        }))
    });
    (unsafe { testType_43(10_()) });
    (unsafe { testType_44(10_()) });
    (unsafe { testType_45(IntComparableWith_long_::new({ Some(10_()) })) });
    (unsafe { testType_46(IntComparableWith_unsigned_long_::new({ Some(10_()) })) });
    (unsafe { testType_47(IntComparableWith_int_::new({ Some(10_()) })) });
    (unsafe { testType_48(10_()) });
    (unsafe { testType_49(10_()) });
    (unsafe { testType_50(IntComparableWith_unsigned_int_::new({ Some(10_()) })) });
    (unsafe { testType_51(10_()) });
    (unsafe { testType_52(IntComparableWith_short_::new({ Some(10_()) })) });
    (unsafe { testType_53(IntComparableWith_unsigned_short_::new({ Some(10_()) })) });
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
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_54() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
