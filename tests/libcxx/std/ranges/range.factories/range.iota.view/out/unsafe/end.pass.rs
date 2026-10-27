extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_add_assign_0(lhs: *mut SomeInt, rhs: *const SomeInt) -> *mut SomeInt {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_1(lhs: *mut SomeInt, rhs: *const SomeInt) -> *mut SomeInt {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_assign_2(lhs: *mut SomeInt, mut rhs: ()) -> *mut SomeInt {
    (*lhs).value_ += rhs;
    return lhs;
}
pub unsafe fn operator_sub_assign_3(lhs: *mut SomeInt, mut rhs: ()) -> *mut SomeInt {
    (*lhs).value_ -= rhs;
    return lhs;
}
pub unsafe fn operator_add_4(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_5(mut lhs: SomeInt, mut rhs: SomeInt) -> () {
    return ((lhs.value_) - (rhs.value_));
}
pub unsafe fn operator_add_6(mut lhs: SomeInt, mut rhs: ()) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs))) });
}
pub unsafe fn operator_sub_7(mut lhs: SomeInt, mut rhs: ()) -> () {
    return ((lhs.value_) - (rhs));
}
pub unsafe fn operator_add_8(mut lhs: (), mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs) + (rhs.value_))) });
}
pub unsafe fn operator_sub_9(mut lhs: (), mut rhs: SomeInt) -> () {
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
pub unsafe fn operator_eq_10(mut lhs: SomeInt, mut rhs: IntComparableWith_SomeInt_) -> () {
    return (unsafe { SomeInt::operator_eq(&lhs, &rhs.value_) });
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
pub unsafe fn operator_eq_11(mut lhs: (), mut rhs: IntComparableWith_long_long_) -> () {
    return ((lhs) == (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_long_long_ {
    pub value_: (),
}
impl IntComparableWith_long_long_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_long_long_ {
    fn default() -> Self {
        unsafe { IntComparableWith_long_long_::new(None) }
    }
}
pub unsafe fn operator_eq_12(mut lhs: (), mut rhs: IntComparableWith_unsigned_long_long_) -> () {
    return ((lhs) == (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_unsigned_long_long_ {
    pub value_: (),
}
impl IntComparableWith_unsigned_long_long_ {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { unimplemented!() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_unsigned_long_long_ {
    fn default() -> Self {
        unsafe { IntComparableWith_unsigned_long_long_::new(None) }
    }
}
pub unsafe fn operator_eq_13(mut lhs: (), mut rhs: IntComparableWith_long_) -> () {
    return ((lhs) == (rhs.value_));
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
pub unsafe fn operator_eq_14(mut lhs: (), mut rhs: IntComparableWith_unsigned_long_) -> () {
    return ((lhs) == (rhs.value_));
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
pub unsafe fn operator_eq_15(mut lhs: (), mut rhs: IntComparableWith_int_) -> () {
    return ((lhs) == (rhs.value_));
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
pub unsafe fn operator_eq_16(mut lhs: (), mut rhs: IntComparableWith_unsigned_int_) -> () {
    return ((lhs) == (rhs.value_));
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
pub unsafe fn operator_eq_17(mut lhs: (), mut rhs: IntComparableWith_short_) -> () {
    return ((lhs) == (rhs.value_));
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
pub unsafe fn operator_eq_18(mut lhs: (), mut rhs: IntComparableWith_unsigned_short_) -> () {
    return ((lhs) == (rhs.value_));
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
pub unsafe fn operator_add_assign_19(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_20(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_21(
    mut lhs: NotIncrementable,
    mut rhs: NotIncrementable,
) -> NotIncrementable {
    return NotIncrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_22(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> () {
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
pub unsafe fn operator_add_assign_23(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_24(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_25(
    mut lhs: NotDecrementable,
    mut rhs: NotDecrementable,
) -> NotDecrementable {
    return NotDecrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_26(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> () {
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
pub unsafe fn testType_27(mut u: SomeInt) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_28(mut u: IntComparableWith_SomeInt_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_29(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_30(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_31(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_32(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_33(mut u: IntComparableWith_long_long_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_34(mut u: IntComparableWith_unsigned_long_long_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_35(mut u: IntComparableWith_long_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_36(mut u: IntComparableWith_unsigned_long_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_37(mut u: IntComparableWith_int_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_38(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_39(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_40(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_41(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_42(mut u: IntComparableWith_unsigned_int_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_43(mut u: ()) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_44(mut u: IntComparableWith_short_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn testType_45(mut u: IntComparableWith_unsigned_short_) {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_46() -> () {
    (unsafe { testType_27(SomeInt::new({ Some(10_()) })) });
    (unsafe {
        testType_28(IntComparableWith_SomeInt_::new({
            Some(SomeInt::new({ Some(10_()) }))
        }))
    });
    (unsafe { testType_29(10_()) });
    (unsafe { testType_30(10_()) });
    (unsafe { testType_31(10_()) });
    (unsafe { testType_32(10_()) });
    (unsafe { testType_33(IntComparableWith_long_long_::new({ Some(10_()) })) });
    (unsafe { testType_34(IntComparableWith_unsigned_long_long_::new({ Some(10_()) })) });
    (unsafe { testType_35(IntComparableWith_long_::new({ Some(10_()) })) });
    (unsafe { testType_36(IntComparableWith_unsigned_long_::new({ Some(10_()) })) });
    (unsafe { testType_37(IntComparableWith_int_::new({ Some(10_()) })) });
    (unsafe { testType_38(10_()) });
    (unsafe { testType_39(10_()) });
    (unsafe { testType_40(10_()) });
    (unsafe { testType_41(10_()) });
    (unsafe { testType_42(IntComparableWith_unsigned_int_::new({ Some(10_()) })) });
    (unsafe { testType_43(10_()) });
    (unsafe { testType_44(IntComparableWith_short_::new({ Some(10_()) })) });
    (unsafe { testType_45(IntComparableWith_unsigned_short_::new({ Some(10_()) })) });
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
    (unsafe { test_46() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
