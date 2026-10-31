extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_lt_0(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) < ((*rhs).value));
}
pub unsafe fn operator_eq_1(lhs: *const LessAndEqComp, rhs: *const LessAndEqComp) -> bool {
    return (((*lhs).value) == ((*rhs).value));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct LessAndEqComp {
    pub value: i32,
}
impl LessAndEqComp {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for LessAndEqComp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_0(self as *const LessAndEqComp, other as *const LessAndEqComp) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const LessAndEqComp, self as *const LessAndEqComp) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for LessAndEqComp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for LessAndEqComp {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_1(self as *const LessAndEqComp, other as *const LessAndEqComp) }
    }
}
impl std::cmp::Eq for LessAndEqComp {}
pub unsafe fn operator_cmp_2(mut _a0: StrongOrder, mut _a1: StrongOrder) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&(_a0.value), &(_a1.value));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct StrongOrder {
    pub value: i32,
}
impl StrongOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for StrongOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_2(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for StrongOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for StrongOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_2(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for StrongOrder {}
pub unsafe fn operator_cmp_3(mut _a0: WeakOrder, mut _a1: WeakOrder) -> () {
    {
        let mut cmp: () = unimplemented!();
        if !(unimplemented!()) {
            return unimplemented!();
        }
    }
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct WeakOrder {
    pub value: i32,
}
impl WeakOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for WeakOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_3(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for WeakOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for WeakOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_cmp_3(self.clone(), other.clone()) == std::cmp::Ordering::Equal }
    }
}
impl std::cmp::Eq for WeakOrder {}
pub unsafe fn operator_cmp_4(mut lhs: PartialOrder, mut rhs: PartialOrder) -> () {
    if ((lhs.value) == (i32::MIN)) || ((rhs.value) == (i32::MIN)) {
        return unimplemented!();
    }
    if ((lhs.value) == (i32::MAX)) || ((rhs.value) == (i32::MAX)) {
        return unimplemented!();
    }
    return unimplemented!();
}
pub unsafe fn operator_eq_5(mut lhs: PartialOrder, mut rhs: PartialOrder) -> bool {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct PartialOrder {
    pub value: i32,
}
impl PartialOrder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { value: v };
        this
    }
}
impl std::cmp::Ord for PartialOrder {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_4(self.clone(), other.clone()) }
    }
}
impl std::cmp::PartialOrd for PartialOrder {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PartialOrder {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for PartialOrder {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NonComparable {}
const _: () = assert!(!(false), "!std::equality_comparable<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorGreaterThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThan<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorLessThanEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorNotEqual<NonComparable>");
const _: () = assert!(!(false), "!HasOperatorSpaceship<NonComparable>");
pub unsafe fn operator_eq_6(
    _a0: *const EqualityComparable,
    _a1: *const EqualityComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct EqualityComparable {
    value_: i32,
}
impl EqualityComparable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::PartialEq for EqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const EqualityComparable,
                other as *const EqualityComparable,
            )
        }
    }
}
impl std::cmp::Eq for EqualityComparable {}
const _: () = assert!(true, "std::equality_comparable<EqualityComparable>");
const _: () = assert!(true, "HasOperatorEqual<EqualityComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<EqualityComparable>");
pub unsafe fn operator_eq_7(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> bool {
    return (((*_a0).value_) == ((*_a1).value_));
}
pub unsafe fn operator_cmp_8(
    _a0: *const ThreeWayComparable,
    _a1: *const ThreeWayComparable,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).value_), &((*_a1).value_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ThreeWayComparable {
    value_: i32,
}
impl ThreeWayComparable {
    pub unsafe fn new(mut value: i32) -> Self {
        let mut this = Self { value_: value };
        this
    }
}
impl std::cmp::Ord for ThreeWayComparable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_8(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::PartialOrd for ThreeWayComparable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for ThreeWayComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_7(
                self as *const ThreeWayComparable,
                other as *const ThreeWayComparable,
            )
        }
    }
}
impl std::cmp::Eq for ThreeWayComparable {}
const _: () = assert!(true, "std::equality_comparable<ThreeWayComparable>");
const _: () = assert!(true, "std::three_way_comparable<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorGreaterThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThan<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorLessThanEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorNotEqual<ThreeWayComparable>");
const _: () = assert!(true, "HasOperatorSpaceship<ThreeWayComparable>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E1 {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ImplicitBool_E2 {
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(bool, to_bool_1)]
pub struct ImplicitBool {
    pub val: bool,
}
impl ImplicitBool {
    pub unsafe fn to_bool_1(&self) -> bool {
        return self.val;
    }
}
pub unsafe fn operator_eq_9(mut lhs: ImplicitBool_E1, mut rhs: ImplicitBool_E2) -> ImplicitBool {
    return ImplicitBool {
        val: ((lhs.x) == (rhs.y)),
    };
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct support_double_move_tracker {
    moved_from_: bool,
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        assert!(!((*other).moved_from_));
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        assert!(!((*other).moved_from_));
        (*other).moved_from_ = true;
        self.moved_from_ = false;
        return &mut (*(self as *mut support_double_move_tracker));
    }
}
impl Default for support_double_move_tracker {
    fn default() -> Self {
        unsafe { support_double_move_tracker::new() }
    }
}
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_output_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_output_iterator_int_ptr_,
    ) -> *mut cpp17_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_output_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp17_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_int_ptr__int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_int_ptr__int_ptr_,
    ) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
}
pub unsafe fn operator_eq_10(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_11(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_5(
        &mut self,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    ) -> *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp17_input_iterator_const_int_ptr__const_int_ptr_));
    }
}
impl std::cmp::PartialEq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
                other as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_12(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_13(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_const_int_ptr_,
    ) -> *mut forward_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_14(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_15(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_const_int_ptr_,
    ) -> *mut bidirectional_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_14(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_16(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_17(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_int_ptr_,
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_16(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.it_.offset((n) as isize));
    }
    pub unsafe fn move_from(_a0: *mut cpp20_random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_random_access_iterator_int_ptr_,
    ) -> *mut cpp20_random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_random_access_iterator_int_ptr_));
    }
}
const _: () = assert!(
    true,
    "std::random_access_iterator<cpp20_random_access_iterator<int*>>"
);
pub unsafe fn operator_eq_18(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_19(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_const_int_ptr_ {
    pub unsafe fn new_1(mut it: *const i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_const_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_const_int_ptr_,
    ) -> *mut contiguous_iterator_const_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_const_int_ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_const_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_18(
                self as *const contiguous_iterator_const_int_ptr_,
                other as *const contiguous_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_9();
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_input_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp20_input_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_input_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_input_iterator_int_ptr_,
    ) -> *mut cpp20_input_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_input_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp20_output_iterator_int_ptr_ {
    pub unsafe fn move_from(_a0: *mut cpp20_output_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut cpp20_output_iterator_int_ptr_,
    ) -> *mut cpp20_output_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut cpp20_output_iterator_int_ptr_));
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
}
const _: () = assert!(
    true,
    "std::output_iterator<cpp20_output_iterator<int*>, int>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct IteratorOpCounts {
    pub increments: usize,
    pub decrements: usize,
    pub zero_moves: usize,
    pub equal_cmps: usize,
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_usize,
                decrements: 0_usize,
                zero_moves: 0_usize,
                equal_cmps: 0_usize,
            }
        }
    }
}
pub unsafe fn operator_eq_20(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> bool {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut i32,
}
impl forward_sized_iterator_int_ptr_ {}
impl std::cmp::PartialEq for forward_sized_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const forward_sized_iterator_int_ptr_,
                other as *const forward_sized_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_sized_iterator_int_ptr_ {}
impl Default for forward_sized_iterator_int_ptr_ {
    fn default() -> Self {
        unsafe {
            forward_sized_iterator_int_ptr_ {
                it_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::forward_iterator<forward_sized_iterator<>>");
const _: () = assert!(
    true,
    "std::sized_sentinel_for<forward_sized_iterator<>, forward_sized_iterator<>>"
);
pub unsafe fn operator_add_21(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_22(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_21(_i, n)
    });
}
pub unsafe fn iter_swap_23(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_24(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_25(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_26(
    lhs: *const adl_Iterator,
    rhs: *const adl_Iterator,
) -> std::cmp::Ordering {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut i32,
    iter_moves_: *mut i32,
    iter_swaps_: *mut i32,
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut i32, mut iter_moves: *mut i32, mut iter_swaps: *mut i32) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut i32, iter_moves: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut i32, iter_swaps: *mut i32) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> i32 {
        assert!(!(self.iter_moves_).is_null());
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> i32 {
        assert!(!(self.iter_swaps_).is_null());
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *mut i32 {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: isize) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> isize {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64
            as isize);
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: isize) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: i32) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_dec();
        return prev;
    }
}
impl std::cmp::Ord for adl_Iterator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_26(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_25(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::Eq for adl_Iterator {}
impl UnsafeCppAdvance for adl_Iterator {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_14();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_16();
    }
}
impl Default for adl_Iterator {
    fn default() -> Self {
        unsafe {
            adl_Iterator {
                ptr_: std::ptr::null_mut(),
                iter_moves_: std::ptr::null_mut(),
                iter_swaps_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rvalue_iterator_int_ptr_ {
    it_: *mut *mut i32,
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> std::cmp::Ordering {
        {
            let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(cmp == std::cmp::Ordering::Equal) {
                return cmp;
            }
        }
        return std::cmp::Ordering::Equal;
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> bool {
        return (((*(self as *const rvalue_iterator_int_ptr_)).it_) == ((*_a0).it_));
    }
}
impl std::cmp::Ord for rvalue_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
        }
    }
}
impl std::cmp::PartialOrd for rvalue_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for rvalue_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            rvalue_iterator_int_ptr_::operator_cmp(self, other as *const rvalue_iterator_int_ptr_)
                == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for rvalue_iterator_int_ptr_ {}
const _: () = assert!(true, "std::random_access_iterator<rvalue_iterator<int*>>");
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*const libc::c_char , new, value)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn test_lexicographical_compare_27(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_28(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_29(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_30(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_31(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_32(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_33(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_34(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_35(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_36(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_37(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_38(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_39(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_40(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_41(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_42(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_43(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_44(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_45(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_46(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_47(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_48(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_49(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_50(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_51(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_52(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_53(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_54(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_55(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_56(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_57(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_58(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_59(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_60(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_61(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_62(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_63(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_64(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_65(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_66(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_67(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_68(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_69(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_70(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_71(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_72(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_73(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_74(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_75(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_76(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_77(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_78(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_79(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_80(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_81(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_82(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_83(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_84(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_85(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_86(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_87(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_88(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_89(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_90(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_91(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_92(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_93(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_94(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_95(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_96(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_97(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_98(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_99(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_100(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_101(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_102(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_103(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_104(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_105(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_106(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_107(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_108(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_109(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_110(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_111(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_112(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_113(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_114(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_115(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_116(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_117(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_118(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_119(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_120(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_121(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_122(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_123(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_124(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_125(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_126(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_127(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_128(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_129(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_130(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_131(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_132(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_133(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_134(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_135(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_136(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_137(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_138(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_139(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_140(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_141(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_142(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_143(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_144(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_145(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_146(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_147(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_148(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_149(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_150(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_151(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_152(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_153(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_154(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_155(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_156(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_157(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_158(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_159(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_160(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_161(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_162(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_163(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_164(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_165(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_166(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_167(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_168(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_169(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_170(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_171(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_172(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_173(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_174(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_175(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_176(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_177(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_178(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_179(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_180(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_181(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_182(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_183(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_184(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_185(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_186(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_187(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_188(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_189(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_190(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_191(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_192(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_193(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_194(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_195(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_196(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_197(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_198(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_199(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_200(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_201(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_202(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_203(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_204(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_205(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_206(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_207(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_208(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_209(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_210(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_211(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_212(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_213(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_214(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_215(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_216(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_217(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_218(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_219(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_220(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_221(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_222(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_223(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_224(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_225(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_226(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_227(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_228(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_229(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_230(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_231(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_232(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_233(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_234(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_235(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_236(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_237(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_238(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_239(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_240(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_241(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_242(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_243(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_244(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_245(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_246(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_247(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_248(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_249(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_250(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_251(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_252(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_253(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_254(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_255(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_256(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_257(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_258(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_259(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_260(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_261(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_262(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_263(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_264(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_265(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_266(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_267(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_268(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_269(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_270(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_271(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_272(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_273(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_274(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_275(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_276(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_277(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_278(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_279(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_280(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_281(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_282(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_283(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_284(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_285(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_286(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_287(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_288(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_289(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_290(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_291(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_292(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_293(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_294(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_295(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_296(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_297(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_298(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_299(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_300(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_301(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_302(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_303(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_304(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_305(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_306(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_307(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_308(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_309(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_310(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_311(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_312(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_313(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_314(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_315(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_316(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_317(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_318(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_319(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_320(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_321(
    mut a: Vec<StrongOrder>,
    mut b: Vec<StrongOrder>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_322(
    mut a: Vec<WeakOrder>,
    mut b: Vec<WeakOrder>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_lexicographical_compare_323(
    mut a: Vec<PartialOrder>,
    mut b: Vec<PartialOrder>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_lexicographical_compare_324(
    mut a: Vec<StrongOrder>,
    mut b: Vec<StrongOrder>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_325(
    mut a: Vec<WeakOrder>,
    mut b: Vec<WeakOrder>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_lexicographical_compare_326(
    mut a: Vec<PartialOrder>,
    mut b: Vec<PartialOrder>,
    mut expected: (),
) {
    let mut result: () = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_given_iterator_types_327() {
    (unsafe {
        test_lexicographical_compare_27(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_28(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_29(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_30(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_30(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_30(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_30(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_31(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_32(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_328() {
    (unsafe {
        test_lexicographical_compare_33(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_34(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_35(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_36(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_36(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_36(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_36(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_37(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_38(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_329() {
    (unsafe {
        test_lexicographical_compare_39(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_40(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_41(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_42(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_42(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_42(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_42(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_43(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_44(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_330() {
    (unsafe {
        test_lexicographical_compare_45(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_46(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_47(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_48(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_48(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_48(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_48(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_49(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_50(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_331() {
    (unsafe {
        test_lexicographical_compare_51(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_52(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_53(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_55(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_56(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_332() {
    (unsafe {
        test_lexicographical_compare_57(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_58(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_59(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_60(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_60(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_60(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_60(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_61(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_62(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_333() {
    (unsafe {
        test_lexicographical_compare_63(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_64(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_65(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_66(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_66(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_66(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_66(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_67(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_68(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_334() {
    (unsafe {
        test_lexicographical_compare_69(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_70(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_71(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_72(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_72(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_72(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_72(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_73(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_74(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_335() {
    (unsafe {
        test_lexicographical_compare_75(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_76(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_77(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_78(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_78(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_78(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_78(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_79(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_80(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_336() {
    (unsafe {
        test_lexicographical_compare_81(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_82(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_83(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_84(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_84(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_84(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_84(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_85(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_86(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_337() {
    (unsafe {
        test_lexicographical_compare_87(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_88(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_89(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_90(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_90(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_90(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_90(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_91(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_92(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_338() {
    (unsafe {
        test_lexicographical_compare_93(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_94(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_95(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_97(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_98(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_339() {
    (unsafe {
        test_lexicographical_compare_99(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_100(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_101(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_102(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_102(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_102(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_102(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_103(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_104(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_340() {
    (unsafe {
        test_lexicographical_compare_105(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_106(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_107(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_108(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_108(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_108(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_108(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_109(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_110(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_341() {
    (unsafe {
        test_lexicographical_compare_111(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_112(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_113(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_114(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_114(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_114(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_114(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_115(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_116(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_342() {
    (unsafe {
        test_lexicographical_compare_117(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_118(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_119(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_120(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_120(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_120(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_120(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_121(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_122(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_343() {
    (unsafe {
        test_lexicographical_compare_123(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_124(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_125(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_126(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_126(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_126(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_126(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_127(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_128(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_344() {
    (unsafe {
        test_lexicographical_compare_129(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_130(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_131(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_132(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_132(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_132(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_132(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_133(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_134(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_345() {
    (unsafe {
        test_lexicographical_compare_135(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_136(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_137(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_139(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_140(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_346() {
    (unsafe {
        test_lexicographical_compare_141(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_142(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_143(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_144(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_144(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_144(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_144(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_145(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_146(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_347() {
    (unsafe {
        test_lexicographical_compare_147(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_148(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_149(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_150(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_150(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_150(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_150(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_151(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_152(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_348() {
    (unsafe {
        test_lexicographical_compare_153(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_154(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_155(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_156(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_156(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_156(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_156(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_157(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_158(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_349() {
    (unsafe {
        test_lexicographical_compare_159(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_160(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_161(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_162(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_162(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_162(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_162(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_163(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_164(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_350() {
    (unsafe {
        test_lexicographical_compare_165(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_166(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_167(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_168(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_168(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_168(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_168(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_169(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_170(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_351() {
    (unsafe {
        test_lexicographical_compare_171(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_172(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_173(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_174(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_174(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_174(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_174(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_175(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_176(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_352() {
    (unsafe {
        test_lexicographical_compare_177(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_178(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_179(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_181(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_182(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_353() {
    (unsafe {
        test_lexicographical_compare_183(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_184(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_185(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_186(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_186(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_186(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_186(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_187(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_188(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_354() {
    (unsafe {
        test_lexicographical_compare_189(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_190(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_191(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_192(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_192(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_192(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_192(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_193(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_194(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_355() {
    (unsafe {
        test_lexicographical_compare_195(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_196(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_197(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_198(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_198(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_198(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_198(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_199(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_200(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_356() {
    (unsafe {
        test_lexicographical_compare_201(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_202(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_203(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_204(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_204(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_204(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_204(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_205(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_206(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_357() {
    (unsafe {
        test_lexicographical_compare_207(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_208(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_209(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_210(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_210(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_210(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_210(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_211(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_212(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_358() {
    (unsafe {
        test_lexicographical_compare_213(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_214(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_215(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_216(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_216(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_216(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_216(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_217(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_218(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_359() {
    (unsafe {
        test_lexicographical_compare_219(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_220(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_221(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_223(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_224(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_360() {
    (unsafe {
        test_lexicographical_compare_225(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_226(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_227(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_228(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_228(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_228(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_228(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_229(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_230(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_361() {
    (unsafe {
        test_lexicographical_compare_231(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_232(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_233(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_234(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_234(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_234(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_234(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_235(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_236(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_362() {
    (unsafe {
        test_lexicographical_compare_237(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_238(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_239(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_240(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_240(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_240(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_240(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_241(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_242(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_363() {
    (unsafe {
        test_lexicographical_compare_243(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_244(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_245(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_246(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_246(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_246(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_246(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_247(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_248(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_364() {
    (unsafe {
        test_lexicographical_compare_249(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_250(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_251(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_252(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_252(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_252(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_252(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_253(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_254(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_365() {
    (unsafe {
        test_lexicographical_compare_255(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_256(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_257(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_258(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_258(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_258(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_258(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_259(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_260(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_366() {
    (unsafe {
        test_lexicographical_compare_261(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_262(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_263(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_265(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_266(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_367() {
    (unsafe {
        test_lexicographical_compare_267(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_268(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_269(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_270(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_270(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_270(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_270(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_271(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_272(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_368() {
    (unsafe {
        test_lexicographical_compare_273(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_274(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_275(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_276(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_276(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_276(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_276(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_277(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_278(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_369() {
    (unsafe {
        test_lexicographical_compare_279(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_280(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_281(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_282(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_282(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_282(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_282(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_283(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_284(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_370() {
    (unsafe {
        test_lexicographical_compare_285(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_286(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_287(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_288(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_288(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_288(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_288(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_289(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_290(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_371() {
    (unsafe {
        test_lexicographical_compare_291(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_292(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_293(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_294(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_294(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_294(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_294(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_295(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_296(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_372() {
    (unsafe {
        test_lexicographical_compare_297(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_298(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_299(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_300(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_300(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_300(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_300(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_301(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_302(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_373() {
    (unsafe {
        test_lexicographical_compare_303(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_304(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_305(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_307(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_308(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_374() {
    (unsafe {
        test_lexicographical_compare_309(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_310(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_311(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_312(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_312(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_312(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_312(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_313(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_314(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_given_iterator_types_375() {
    (unsafe {
        test_lexicographical_compare_315(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_316(
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            Vec::from([0, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_317(
            Vec::from([0, 1]),
            std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_318(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_318(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_318(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_318(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_319(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_320(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            std::cmp::Ordering::Greater,
        )
    });
}
pub unsafe fn test_iterator_types1_376() {
    (unsafe { test_given_iterator_types_327() });
    (unsafe { test_given_iterator_types_328() });
    (unsafe { test_given_iterator_types_329() });
    (unsafe { test_given_iterator_types_330() });
    (unsafe { test_given_iterator_types_331() });
    (unsafe { test_given_iterator_types_332() });
    (unsafe { test_given_iterator_types_333() });
}
pub unsafe fn test_iterator_types1_377() {
    (unsafe { test_given_iterator_types_334() });
    (unsafe { test_given_iterator_types_335() });
    (unsafe { test_given_iterator_types_336() });
    (unsafe { test_given_iterator_types_337() });
    (unsafe { test_given_iterator_types_338() });
    (unsafe { test_given_iterator_types_339() });
    (unsafe { test_given_iterator_types_340() });
}
pub unsafe fn test_iterator_types1_378() {
    (unsafe { test_given_iterator_types_341() });
    (unsafe { test_given_iterator_types_342() });
    (unsafe { test_given_iterator_types_343() });
    (unsafe { test_given_iterator_types_344() });
    (unsafe { test_given_iterator_types_345() });
    (unsafe { test_given_iterator_types_346() });
    (unsafe { test_given_iterator_types_347() });
}
pub unsafe fn test_iterator_types1_379() {
    (unsafe { test_given_iterator_types_348() });
    (unsafe { test_given_iterator_types_349() });
    (unsafe { test_given_iterator_types_350() });
    (unsafe { test_given_iterator_types_351() });
    (unsafe { test_given_iterator_types_352() });
    (unsafe { test_given_iterator_types_353() });
    (unsafe { test_given_iterator_types_354() });
}
pub unsafe fn test_iterator_types1_380() {
    (unsafe { test_given_iterator_types_355() });
    (unsafe { test_given_iterator_types_356() });
    (unsafe { test_given_iterator_types_357() });
    (unsafe { test_given_iterator_types_358() });
    (unsafe { test_given_iterator_types_359() });
    (unsafe { test_given_iterator_types_360() });
    (unsafe { test_given_iterator_types_361() });
}
pub unsafe fn test_iterator_types1_381() {
    (unsafe { test_given_iterator_types_362() });
    (unsafe { test_given_iterator_types_363() });
    (unsafe { test_given_iterator_types_364() });
    (unsafe { test_given_iterator_types_365() });
    (unsafe { test_given_iterator_types_366() });
    (unsafe { test_given_iterator_types_367() });
    (unsafe { test_given_iterator_types_368() });
}
pub unsafe fn test_iterator_types1_382() {
    (unsafe { test_given_iterator_types_369() });
    (unsafe { test_given_iterator_types_370() });
    (unsafe { test_given_iterator_types_371() });
    (unsafe { test_given_iterator_types_372() });
    (unsafe { test_given_iterator_types_373() });
    (unsafe { test_given_iterator_types_374() });
    (unsafe { test_given_iterator_types_375() });
}
pub unsafe fn test_iterator_types_383() {
    (unsafe { test_iterator_types1_376() });
    (unsafe { test_iterator_types1_377() });
    (unsafe { test_iterator_types1_378() });
    (unsafe { test_iterator_types1_379() });
    (unsafe { test_iterator_types1_380() });
    (unsafe { test_iterator_types1_381() });
    (unsafe { test_iterator_types1_382() });
}
pub unsafe fn test_comparison_categories_384() {
    (unsafe {
        test_lexicographical_compare_321(
            Vec::from([StrongOrder::new({ 0 }), StrongOrder::new({ 1 })]),
            Vec::from([StrongOrder::new({ 1 }), StrongOrder::new({ 1 })]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_322(
            Vec::from([WeakOrder::new({ 0 }), WeakOrder::new({ 1 })]),
            Vec::from([WeakOrder::new({ 1 }), WeakOrder::new({ 1 })]),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_323(
            Vec::from([PartialOrder::new({ 0 }), PartialOrder::new({ 1 })]),
            Vec::from([PartialOrder::new({ 1 }), PartialOrder::new({ 1 })]),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_324(
            Vec::from([StrongOrder::new({ 0 }), StrongOrder::new({ 1 })]),
            Vec::from([
                StrongOrder::new({ 0 }),
                StrongOrder::new({ 1 }),
                StrongOrder::new({ 2 }),
            ]),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_325(
            Vec::from([WeakOrder::new({ 0 }), WeakOrder::new({ 1 })]),
            Vec::from([
                WeakOrder::new({ 0 }),
                WeakOrder::new({ 1 }),
                WeakOrder::new({ 2 }),
            ]),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_326(
            Vec::from([PartialOrder::new({ 0 }), PartialOrder::new({ 1 })]),
            Vec::from([
                PartialOrder::new({ 0 }),
                PartialOrder::new({ 1 }),
                PartialOrder::new({ 2 }),
            ]),
            unimplemented!(),
        )
    });
    (unsafe {
        test_lexicographical_compare_326(
            Vec::from([PartialOrder::new({ i32::MIN }), PartialOrder::new({ 1 })]),
            Vec::from([
                PartialOrder::new({ 0 }),
                PartialOrder::new({ 1 }),
                PartialOrder::new({ 2 }),
            ]),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_proxy_iterators_385() {
    let mut vec_: Vec<bool> = unimplemented!();
    let mut result: std::cmp::Ordering = unimplemented!();
    assert!(result == std::cmp::Ordering::Equal);
}
pub unsafe fn test_386() -> bool {
    (unsafe { test_iterator_types_383() });
    (unsafe { test_comparison_categories_384() });
    (unsafe { test_proxy_iterators_385() });
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
    (unsafe { test_386() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
