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
pub unsafe fn operator_add_assign_10(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_11(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_12(
    mut lhs: NotIncrementable,
    mut rhs: NotIncrementable,
) -> NotIncrementable {
    return NotIncrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_13(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> () {
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
pub unsafe fn operator_add_assign_14(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_15(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_16(
    mut lhs: NotDecrementable,
    mut rhs: NotDecrementable,
) -> NotDecrementable {
    return NotDecrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_17(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> () {
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
pub unsafe fn operator_add_assign_18(
    lhs: *mut NotNoexceptCopy,
    rhs: *const NotNoexceptCopy,
) -> *mut NotNoexceptCopy {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_19(
    lhs: *mut NotNoexceptCopy,
    rhs: *const NotNoexceptCopy,
) -> *mut NotNoexceptCopy {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_20(
    mut lhs: NotNoexceptCopy,
    mut rhs: NotNoexceptCopy,
) -> NotNoexceptCopy {
    return NotNoexceptCopy::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_21(mut lhs: NotNoexceptCopy, mut rhs: NotNoexceptCopy) -> () {
    return ((lhs.value_) - (rhs.value_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NotNoexceptCopy {
    pub value_: (),
}
impl NotNoexceptCopy {
    pub unsafe fn new(mut value: Option<()>) -> Self {
        let mut value: () = value.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn copy_from(other: *const NotNoexceptCopy) -> Self {
        let mut this = Self {
            value_: (*other).value_,
        };
        this
    }
    pub unsafe fn operator_eq(&self, _a0: *const NotNoexceptCopy) -> () {
        return (((*(self as *const NotNoexceptCopy)).value_) == ((*_a0).value_));
    }
    pub unsafe fn operator_inc_4(&mut self) -> *mut NotNoexceptCopy {
        self.value_.prefix_inc();
        return &mut (*(self as *mut NotNoexceptCopy));
    }
    pub unsafe fn operator_post_inc_5(&mut self, mut _a0: ()) {
        self.value_.prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NotNoexceptCopy) -> *mut NotNoexceptCopy {
        self.value_ = (*_a0).value_;
        return &mut (*(self as *mut NotNoexceptCopy));
    }
}
impl std::cmp::PartialEq for NotNoexceptCopy {
    fn eq(&self, other: &Self) -> bool {
        unsafe { NotNoexceptCopy::operator_eq(self, other as *const NotNoexceptCopy) }
    }
}
impl std::cmp::Eq for NotNoexceptCopy {}
impl Clone for NotNoexceptCopy {
    fn clone(&self) -> Self {
        unsafe { NotNoexceptCopy::copy_from(self as *const NotNoexceptCopy) }
    }
}
impl Default for NotNoexceptCopy {
    fn default() -> Self {
        unsafe { NotNoexceptCopy::new(None) }
    }
}
pub unsafe fn testType_22() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_23() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_24() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_25() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_26() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_27() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_28() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_29() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_30() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_31() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_32() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn testType_33() {
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
        const _: () = assert!(true, "noexcept(*iter) == !std::same_as<T, NotNoexceptCopy>");;
    }
    {
        let mut io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 10_();
        'loop_: while ((i) < (100_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
    }
}
pub unsafe fn test_34() -> () {
    (unsafe { testType_22() });
    (unsafe { testType_23() });
    (unsafe { testType_24() });
    (unsafe { testType_25() });
    (unsafe { testType_26() });
    (unsafe { testType_27() });
    (unsafe { testType_28() });
    (unsafe { testType_29() });
    (unsafe { testType_30() });
    (unsafe { testType_31() });
    (unsafe { testType_32() });
    (unsafe { testType_33() });
    {
        let io: () = unimplemented!();
        let mut iter: () = (unsafe { io.begin() });
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            unimplemented!();
            {
                i.prefix_inc();
                unimplemented!()
            };
        }
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
    (unsafe { test_34() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
