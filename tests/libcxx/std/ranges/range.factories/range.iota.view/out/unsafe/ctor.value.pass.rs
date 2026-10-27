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
pub unsafe fn operator_eq_18(mut lhs: SomeIntComparable, mut rhs: SomeIntComparable) -> () {
    return (unsafe {
        let _arg0: *const SomeInt = &rhs.value_;
        SomeInt::operator_eq(&lhs.value_, _arg0)
    });
}
pub unsafe fn operator_eq_19(mut lhs: SomeIntComparable, mut rhs: SomeInt) -> () {
    return (unsafe { SomeInt::operator_eq(&lhs.value_, &rhs) });
}
pub unsafe fn operator_eq_20(mut lhs: SomeInt, mut rhs: SomeIntComparable) -> () {
    return (unsafe { SomeInt::operator_eq(&lhs, &rhs.value_) });
}
pub unsafe fn operator_sub_21(mut lhs: SomeIntComparable, mut rhs: SomeIntComparable) -> () {
    return (unsafe {
        let _lhs: SomeInt = lhs.value_;
        let _rhs: SomeInt = rhs.value_;
        operator_sub_5(_lhs, _rhs)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SomeIntComparable {
    pub value_: SomeInt,
}
impl SomeIntComparable {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            value_: SomeInt::new({ Some(10_()) }),
        };
        this
    }
    pub unsafe fn operator_inc_2(&mut self) -> *mut SomeIntComparable {
        (unsafe { SomeInt::operator_inc_3(&mut self.value_) });
        return &mut (*(self as *mut SomeIntComparable));
    }
    pub unsafe fn operator_post_inc_3(&mut self, mut _a0: ()) -> SomeIntComparable {
        let mut tmp: SomeIntComparable = (*(self as *mut SomeIntComparable));
        (unsafe { SomeInt::operator_inc_3(&mut self.value_) });
        return tmp;
    }
    pub unsafe fn operator_dec(&mut self) -> SomeIntComparable {
        (unsafe { SomeInt::operator_dec_5(&mut self.value_) });
        return (*(self as *mut SomeIntComparable));
    }
}
impl std::cmp::PartialEq for SomeIntComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_18(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for SomeIntComparable {}
impl Default for SomeIntComparable {
    fn default() -> Self {
        unsafe { SomeIntComparable::new() }
    }
}
pub unsafe fn test_22() -> () {
    {
        let mut io: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(
            true,
            "std::same_as<decltype(io.end()), std::unreachable_sentinel_t>"
        );;
    }
    {
        let mut io: () = unimplemented!();
        unimplemented!();
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
    (unsafe { test_22() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
