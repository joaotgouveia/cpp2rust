extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            (unsafe { f.spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            (unsafe { f.spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            (unsafe { f.spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            (unsafe { f.spec::<fn()>(11).call() });
            0_()
        };
        let _arg12: () = {
            (unsafe { f.spec::<fn()>(12).call() });
            0_()
        };
        let _arg13: () = {
            (unsafe { f.spec::<fn()>(13).call() });
            0_()
        };
        let _arg14: () = {
            (unsafe { f.spec::<fn()>(14).call() });
            0_()
        };
        let _arg15: () = {
            (unsafe { f.spec::<fn()>(15).call() });
            0_()
        };
        let _arg16: () = {
            (unsafe { f.spec::<fn()>(16).call() });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16,
        )
    });
}
pub unsafe fn for_each_2(
    mut _a0 : types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_,
    mut f: FnPtr<Generic>,
) {
    (unsafe {
        let _arg0: () = {
            (unsafe { f.spec::<fn()>(0).call() });
            0_()
        };
        let _arg1: () = {
            (unsafe { f.spec::<fn()>(1).call() });
            0_()
        };
        let _arg2: () = {
            (unsafe { f.spec::<fn()>(2).call() });
            0_()
        };
        let _arg3: () = {
            (unsafe { f.spec::<fn()>(3).call() });
            0_()
        };
        let _arg4: () = {
            (unsafe { f.spec::<fn()>(4).call() });
            0_()
        };
        let _arg5: () = {
            (unsafe { f.spec::<fn()>(5).call() });
            0_()
        };
        let _arg6: () = {
            (unsafe { f.spec::<fn()>(6).call() });
            0_()
        };
        let _arg7: () = {
            (unsafe { f.spec::<fn()>(7).call() });
            0_()
        };
        let _arg8: () = {
            (unsafe { f.spec::<fn()>(8).call() });
            0_()
        };
        let _arg9: () = {
            (unsafe { f.spec::<fn()>(9).call() });
            0_()
        };
        let _arg10: () = {
            (unsafe { f.spec::<fn()>(10).call() });
            0_()
        };
        let _arg11: () = {
            (unsafe { f.spec::<fn()>(11).call() });
            0_()
        };
        let _arg12: () = {
            (unsafe { f.spec::<fn()>(12).call() });
            0_()
        };
        let _arg13: () = {
            (unsafe { f.spec::<fn()>(13).call() });
            0_()
        };
        let _arg14: () = {
            (unsafe { f.spec::<fn()>(14).call() });
            0_()
        };
        let _arg15: () = {
            (unsafe { f.spec::<fn()>(15).call() });
            0_()
        };
        let _arg16: () = {
            (unsafe { f.spec::<fn()>(16).call() });
            0_()
        };
        swallow_1(
            _arg0, _arg1, _arg2, _arg3, _arg4, _arg5, _arg6, _arg7, _arg8, _arg9, _arg10, _arg11,
            _arg12, _arg13, _arg14, _arg15, _arg16,
        )
    });
}
pub unsafe fn swallow_1(
    mut _a0: (),
    mut _a1: (),
    mut _a2: (),
    mut _a3: (),
    mut _a4: (),
    mut _a5: (),
    mut _a6: (),
    mut _a7: (),
    mut _a8: (),
    mut _a9: (),
    mut _a10: (),
    mut _a11: (),
    mut _a12: (),
    mut _a13: (),
    mut _a14: (),
    mut _a15: (),
    mut _a16: (),
) {
}
pub unsafe fn operator_add_assign_3(lhs: *mut SomeInt, rhs: *const SomeInt) -> *mut SomeInt {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_4(lhs: *mut SomeInt, rhs: *const SomeInt) -> *mut SomeInt {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_assign_5(lhs: *mut SomeInt, mut rhs: ()) -> *mut SomeInt {
    (*lhs).value_ += rhs;
    return lhs;
}
pub unsafe fn operator_sub_assign_6(lhs: *mut SomeInt, mut rhs: ()) -> *mut SomeInt {
    (*lhs).value_ -= rhs;
    return lhs;
}
pub unsafe fn operator_add_7(mut lhs: SomeInt, mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_8(mut lhs: SomeInt, mut rhs: SomeInt) -> () {
    return ((lhs.value_) - (rhs.value_));
}
pub unsafe fn operator_add_9(mut lhs: SomeInt, mut rhs: ()) -> SomeInt {
    return SomeInt::new({ Some(((lhs.value_) + (rhs))) });
}
pub unsafe fn operator_sub_10(mut lhs: SomeInt, mut rhs: ()) -> () {
    return ((lhs.value_) - (rhs));
}
pub unsafe fn operator_add_11(mut lhs: (), mut rhs: SomeInt) -> SomeInt {
    return SomeInt::new({ Some(((lhs) + (rhs.value_))) });
}
pub unsafe fn operator_sub_12(mut lhs: (), mut rhs: SomeInt) -> () {
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
pub unsafe fn operator_add_assign_13(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_14(
    lhs: *mut NotIncrementable,
    rhs: *const NotIncrementable,
) -> *mut NotIncrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_15(
    mut lhs: NotIncrementable,
    mut rhs: NotIncrementable,
) -> NotIncrementable {
    return NotIncrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_16(mut lhs: NotIncrementable, mut rhs: NotIncrementable) -> () {
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
pub unsafe fn operator_add_assign_17(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ += (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_sub_assign_18(
    lhs: *mut NotDecrementable,
    rhs: *const NotDecrementable,
) -> *mut NotDecrementable {
    (*lhs).value_ -= (*rhs).value_;
    return lhs;
}
pub unsafe fn operator_add_19(
    mut lhs: NotDecrementable,
    mut rhs: NotDecrementable,
) -> NotDecrementable {
    return NotDecrementable::new({ Some(((lhs.value_) + (rhs.value_))) });
}
pub unsafe fn operator_sub_20(mut lhs: NotDecrementable, mut rhs: NotDecrementable) -> () {
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
pub unsafe fn test_21() -> () {
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {}
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        let imax: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        let io: () = unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "std::ranges::sized_range<std::ranges::iota_view<short, short>>"
        );;
        let mut io: () = unimplemented!();
        let mut sz: () = unimplemented!();
        unimplemented!();
    }
    {
        (unsafe {
            for_each_0 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { ( unsafe { for_each_2 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_22 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_23 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_24 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_25 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_26 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_27 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_28 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_29 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_30 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_31 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_32 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_33 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_34 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_35 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_36 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } , | | { ( unsafe { for_each_37 ( <types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() , lambda_unsafe! ( Generic , { } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } , | | { const _: () = assert!(! ( false ) , "!HasSize<std::ranges::iota_view<IntegerLikeT, bool>>"); ;
 ;
 } ) , ) } )  ;
 } ) , )
        });
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
    (unsafe { test_21() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
