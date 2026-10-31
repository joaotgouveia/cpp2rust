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
pub unsafe fn operator_add_assign_2(lhs: *mut SomeInt, mut rhs: i32) -> *mut SomeInt {
    (*lhs).value_ += rhs;
    return lhs;
}
pub unsafe fn operator_sub_assign_3(lhs: *mut SomeInt, mut rhs: i32) -> *mut SomeInt {
    (*lhs).value_ -= rhs;
    return lhs;
}
pub unsafe fn operator_add_4(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_5(mut lhs: SomeInt, mut rhs: SomeInt) -> i32 {
    return ((lhs.value_) - (rhs.value_));
}
pub unsafe fn operator_add_6(mut lhs: SomeInt, mut rhs: i32) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs))) });
}
pub unsafe fn operator_sub_7(mut lhs: SomeInt, mut rhs: i32) -> i32 {
    return ((lhs.value_) - (rhs));
}
pub unsafe fn operator_add_8(mut lhs: i32, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs) + (rhs.value_))) });
}
pub unsafe fn operator_sub_9(mut lhs: i32, mut rhs: SomeInt) -> i32 {
    return ((lhs) - (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SomeInt {
    pub value_: i32,
}
impl SomeInt {
    pub unsafe fn new(mut value: Option<i32>) -> Self {
        let mut value: i32 = value.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn operator_cmp(&self, _a0: *const SomeInt) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering =
                std::cmp::Ord::cmp(&((*(self as *const SomeInt)).value_), &((*_a0).value_));
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_inc_3(&mut self) -> *mut SomeInt {
        self.value_.prefix_inc();
        return &mut (*(self as *mut SomeInt));
    }
    pub unsafe fn operator_post_inc_4(&mut self, mut _a0: i32) -> SomeInt {
        let mut tmp: SomeInt = (*(self as *mut SomeInt));
        self.value_.prefix_inc();
        return tmp;
    }
    pub unsafe fn operator_dec_5(&mut self) -> *mut SomeInt {
        self.value_.prefix_dec();
        return &mut (*(self as *mut SomeInt));
    }
    pub unsafe fn operator_post_dec_6(&mut self, mut _a0: i32) -> SomeInt {
        let mut tmp: SomeInt = (*(self as *mut SomeInt));
        self.value_.prefix_dec();
        return tmp;
    }
    pub unsafe fn operator_eq(&self, _a0: *const SomeInt) -> bool {
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
impl UnsafeCppAdvance for SomeInt {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_3();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_5();
    }
}
impl Default for SomeInt {
    fn default() -> Self {
        unsafe { SomeInt::new(None) }
    }
}
pub unsafe fn operator_eq_10(
    mut lhs: Int42_DefaultTo42_,
    mut rhs: IntComparableWith_Int42_DefaultTo42__,
) -> bool {
    return (unsafe { Int42_DefaultTo42_::operator_eq(&lhs, &rhs.value_) });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IntComparableWith_Int42_DefaultTo42__ {
    pub value_: Int42_DefaultTo42_,
}
impl IntComparableWith_Int42_DefaultTo42__ {
    pub unsafe fn new(mut value: Option<Int42_DefaultTo42_>) -> Self {
        let mut value: Int42_DefaultTo42_ =
            value.unwrap_or_else(|| unsafe { Int42_DefaultTo42_::new_1() });
        let mut this = Self { value_: value };
        this
    }
}
impl Default for IntComparableWith_Int42_DefaultTo42__ {
    fn default() -> Self {
        unsafe { IntComparableWith_Int42_DefaultTo42__::new(None) }
    }
}
pub unsafe fn operator_add_assign_11(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_12(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_13(
    mut lhs: NotIncrementable,
    mut rhs: NotIncrementable,
) -> NotIncrementable {
    return NotIncrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_14(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> i32 {
    return ((lhs.value_) - (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NotIncrementable {
    pub value_: i32,
}
impl NotIncrementable {
    pub unsafe fn new(mut value: Option<i32>) -> Self {
        let mut value: i32 = value.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn operator_eq(&self, _a0: *const NotIncrementable) -> bool {
        return (((*(self as *const NotIncrementable)).value_) == ((*_a0).value_));
    }
    pub unsafe fn operator_inc_3(&mut self) -> *mut NotIncrementable {
        self.value_.prefix_inc();
        return &mut (*(self as *mut NotIncrementable));
    }
    pub unsafe fn operator_post_inc_4(&mut self, mut _a0: i32) {
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
impl UnsafeCppAdvance for NotIncrementable {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_3();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec();
    }
}
impl Default for NotIncrementable {
    fn default() -> Self {
        unsafe { NotIncrementable::new(None) }
    }
}
const _: () = assert!(!(false), "!std::incrementable<NotIncrementable>");
pub unsafe fn operator_add_assign_15(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_16(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_17(
    mut lhs: NotDecrementable,
    mut rhs: NotDecrementable,
) -> NotDecrementable {
    return NotDecrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_18(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> i32 {
    return ((lhs.value_) - (rhs.value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct NotDecrementable {
    pub value_: i32,
}
impl NotDecrementable {
    pub unsafe fn new(mut value: Option<i32>) -> Self {
        let mut value: i32 = value.unwrap_or_else(|| unsafe { 0 });
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn operator_eq(&self, _a0: *const NotDecrementable) -> bool {
        return (((*(self as *const NotDecrementable)).value_) == ((*_a0).value_));
    }
    pub unsafe fn operator_inc_3(&mut self) -> *mut NotDecrementable {
        self.value_.prefix_inc();
        return &mut (*(self as *mut NotDecrementable));
    }
    pub unsafe fn operator_post_inc_4(&mut self, mut _a0: i32) {
        self.value_.prefix_inc();
    }
}
impl std::cmp::PartialEq for NotDecrementable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { NotDecrementable::operator_eq(self, other as *const NotDecrementable) }
    }
}
impl std::cmp::Eq for NotDecrementable {}
impl UnsafeCppAdvance for NotDecrementable {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_3();
    }
}
impl Default for NotDecrementable {
    fn default() -> Self {
        unsafe { NotDecrementable::new(None) }
    }
}
pub type CtorKind = u32;
pub const CtorKind_DefaultTo42: CtorKind = 0;
pub const CtorKind_ValueCtor: CtorKind = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Int42_DefaultTo42_ {
    pub value_: i32,
}
impl Int42_DefaultTo42_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { value_: 42 };
        this
    }
    pub unsafe fn operator_eq(&self, _a0: *const Int42_DefaultTo42_) -> bool {
        return (((*(self as *const Int42_DefaultTo42_)).value_) == ((*_a0).value_));
    }
}
impl std::cmp::PartialEq for Int42_DefaultTo42_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { Int42_DefaultTo42_::operator_eq(self, other as *const Int42_DefaultTo42_) }
    }
}
impl std::cmp::Eq for Int42_DefaultTo42_ {}
impl Default for Int42_DefaultTo42_ {
    fn default() -> Self {
        unsafe { Int42_DefaultTo42_::new_1() }
    }
}
pub unsafe fn test_19() -> bool {
    assert!(unimplemented!() == unimplemented!());
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_19() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
