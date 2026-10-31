extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
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
pub struct types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr____types_type_list_bidirectional_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr____types_type_list_forward_iterator_int_ptr___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr____types_type_list_cpp17_input_iterator_int_ptr__int_ptr___
{}
pub unsafe fn for_each_0(
    mut _a0 : types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__,
    mut f: Test,
) {
    (unsafe {
        let _arg0: i32 = {
            (unsafe { Test::operator_call_1_int_ptr(&mut f) });
            0
        };
        let _arg1: i32 = {
            (unsafe { Test::operator_call_1_class_contiguous_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg2: i32 = {
            (unsafe { Test::operator_call_1_class_random_access_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg3: i32 = {
            (unsafe { Test::operator_call_1_class_bidirectional_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg4: i32 = {
            (unsafe { Test::operator_call_1_class_forward_iterator_int_ptr_(&mut f) });
            0
        };
        let _arg5: i32 = {
            (unsafe { Test::operator_call_1_class_cpp17_input_iterator_int_ptr_(&mut f) });
            0
        };
        swallow_1(_arg0, _arg1, _arg2, _arg3, _arg4, _arg5)
    });
}
pub unsafe fn swallow_1(
    mut _a0: i32,
    mut _a1: i32,
    mut _a2: i32,
    mut _a3: i32,
    mut _a4: i32,
    mut _a5: i32,
) {
}
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
pub unsafe fn operator_eq_2(
    x: *const cpp17_input_iterator_int_ptr__int_ptr_,
    y: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_3(
    x: *const cpp17_input_iterator_int_ptr__int_ptr_,
    y: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn new(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_5(&mut self) -> *mut cpp17_input_iterator_int_ptr__int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut cpp17_input_iterator_int_ptr__int_ptr_));
    }
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
impl std::cmp::PartialEq for cpp17_input_iterator_int_ptr__int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const cpp17_input_iterator_int_ptr__int_ptr_,
                other as *const cpp17_input_iterator_int_ptr__int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_int_ptr__int_ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_int_ptr__int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_4(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_5(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_int_ptr_,
    ) -> *mut forward_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const forward_iterator_int_ptr_,
                other as *const forward_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_int_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_6(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_7(
    x: *const bidirectional_iterator_int_ptr_,
    y: *const bidirectional_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct bidirectional_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut bidirectional_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut bidirectional_iterator_int_ptr_,
    ) -> *mut bidirectional_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut bidirectional_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for bidirectional_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_6(
                self as *const bidirectional_iterator_int_ptr_,
                other as *const bidirectional_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_int_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_8(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_9(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_int_ptr_,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_8(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_int_ptr_ {
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
pub unsafe fn operator_eq_10(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_11(
    x: *const contiguous_iterator_int_ptr_,
    y: *const contiguous_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct contiguous_iterator_int_ptr_ {
    it_: *mut i32,
    tracker_: support_double_move_tracker,
}
impl contiguous_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut i32) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_9(&mut self) -> *mut contiguous_iterator_int_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut contiguous_iterator_int_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut contiguous_iterator_int_ptr_,
    ) -> *mut contiguous_iterator_int_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut contiguous_iterator_int_ptr_));
    }
}
impl std::cmp::PartialEq for contiguous_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const contiguous_iterator_int_ptr_,
                other as *const contiguous_iterator_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_int_ptr_ {}
impl UnsafeCppAdvance for contiguous_iterator_int_ptr_ {
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
pub unsafe fn operator_eq_12(
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
            operator_eq_12(
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
pub unsafe fn operator_add_13(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_14(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_13(_i, n)
    });
}
pub unsafe fn iter_swap_15(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_16(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_17(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_18(
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
        unsafe { operator_cmp_18(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_17(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub unsafe fn check_19(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_20(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_21(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_22(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_23(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_24(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_25(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_26(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_27(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_28(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_29(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_30(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_31(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_32(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_33(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_34(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_35(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_36(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_37(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_38(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_39(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_40(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_41(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_42(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    {}
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_43(
    mut lhs: Vec<NonTrivialMod4Comp>,
    mut rhs: Vec<NonTrivialMod4Comp>,
    mut offset: usize,
) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_44(
    mut lhs: Vec<TriviallyEqualityComparable>,
    mut rhs: Vec<TriviallyEqualityComparable>,
    mut offset: usize,
) {
    {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_45(mut lhs: Vec<libc::c_char>, mut rhs: Vec<libc::c_char>, mut offset: usize) {
    if ((lhs.len()) == (rhs.len())) {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn check_46(mut lhs: Vec<i32>, mut rhs: Vec<i32>, mut offset: usize) {
    if ((lhs.len()) == (rhs.len())) {
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    assert!(unimplemented!());
    assert!(unimplemented!());
}
pub unsafe fn operator_eq_47(
    lhs: *const NonTrivialMod4Comp,
    rhs: *const NonTrivialMod4Comp,
) -> bool {
    return ((((*lhs).i_) % (4)) == (((*rhs).i_) % (4)));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NonTrivialMod4Comp {
    pub i_: i32,
}
impl NonTrivialMod4Comp {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn move_from(other: *mut NonTrivialMod4Comp) -> Self {
        let mut this = Self { i_: (*other).i_ };
        (*other).i_ = 0;
        this
    }
}
impl std::cmp::PartialEq for NonTrivialMod4Comp {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_47(
                self as *const NonTrivialMod4Comp,
                other as *const NonTrivialMod4Comp,
            )
        }
    }
}
impl std::cmp::Eq for NonTrivialMod4Comp {}
pub unsafe fn operator_eq_48(
    mut _a0: TriviallyEqualityComparable,
    mut _a1: TriviallyEqualityComparable,
) -> bool {
    return ((_a0.i_) == (_a1.i_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct TriviallyEqualityComparable {
    pub i_: i32,
}
impl TriviallyEqualityComparable {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self { i_: i };
        this
    }
}
impl std::cmp::PartialEq for TriviallyEqualityComparable {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_48(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for TriviallyEqualityComparable {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ModTwoComp {}
impl ModTwoComp {
    pub unsafe fn operator_call(&mut self, mut lhs: i32, mut rhs: i32) -> bool {
        return (((lhs) % (2)) == ((rhs) % (2)));
    }
}
pub unsafe fn test_49() -> bool {
    {
        let mut lhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let mut rhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_19(lhs.clone(), rhs.clone(), 0_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_20(lhs.clone(), rhs.clone(), 8_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_20(lhs.clone(), rhs.clone(), 3_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1]);
        (unsafe { check_21(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        (unsafe { check_22(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 2, 3, 4]);
        let mut rhs: Vec<i32> = Vec::from([0, 0, 4, 4]);
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_50() -> bool {
    {
        let mut lhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let mut rhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_23(lhs.clone(), rhs.clone(), 0_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_24(lhs.clone(), rhs.clone(), 8_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_24(lhs.clone(), rhs.clone(), 3_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1]);
        (unsafe { check_25(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        (unsafe { check_26(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 2, 3, 4]);
        let mut rhs: Vec<i32> = Vec::from([0, 0, 4, 4]);
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_51() -> bool {
    {
        let mut lhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let mut rhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_27(lhs.clone(), rhs.clone(), 0_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_28(lhs.clone(), rhs.clone(), 8_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_28(lhs.clone(), rhs.clone(), 3_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1]);
        (unsafe { check_29(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        (unsafe { check_30(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 2, 3, 4]);
        let mut rhs: Vec<i32> = Vec::from([0, 0, 4, 4]);
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_52() -> bool {
    {
        let mut lhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let mut rhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_31(lhs.clone(), rhs.clone(), 0_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_32(lhs.clone(), rhs.clone(), 8_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_32(lhs.clone(), rhs.clone(), 3_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1]);
        (unsafe { check_33(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        (unsafe { check_34(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 2, 3, 4]);
        let mut rhs: Vec<i32> = Vec::from([0, 0, 4, 4]);
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_53() -> bool {
    {
        let mut lhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let mut rhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_35(lhs.clone(), rhs.clone(), 0_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_36(lhs.clone(), rhs.clone(), 8_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_36(lhs.clone(), rhs.clone(), 3_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1]);
        (unsafe { check_37(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        (unsafe { check_38(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 2, 3, 4]);
        let mut rhs: Vec<i32> = Vec::from([0, 0, 4, 4]);
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
pub unsafe fn test_54() -> bool {
    {
        let mut lhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        let mut rhs: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_39(lhs.clone(), rhs.clone(), 0_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_40(lhs.clone(), rhs.clone(), 8_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 3, 0, 1, 2, 3]);
        (unsafe { check_40(lhs.clone(), rhs.clone(), 3_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        let mut rhs: Vec<i32> = Vec::from([0, 1]);
        (unsafe { check_41(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 1]);
        let mut rhs: Vec<i32> = Vec::from([0, 1, 2, 2, 0, 1, 2, 3]);
        (unsafe { check_42(lhs.clone(), rhs.clone(), 2_usize) });
    }
    {
        let mut lhs: Vec<i32> = Vec::from([0, 2, 3, 4]);
        let mut rhs: Vec<i32> = Vec::from([0, 0, 4, 4]);
        assert!(unimplemented!());
        assert!(unimplemented!());
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Test {}
impl Test {
    pub unsafe fn operator_call_1_int_ptr(&mut self) {
        (unsafe { test_49() });
    }
    pub unsafe fn operator_call_1_class_contiguous_iterator_int_ptr_(&mut self) {
        (unsafe { test_50() });
    }
    pub unsafe fn operator_call_1_class_random_access_iterator_int_ptr_(&mut self) {
        (unsafe { test_51() });
    }
    pub unsafe fn operator_call_1_class_bidirectional_iterator_int_ptr_(&mut self) {
        (unsafe { test_52() });
    }
    pub unsafe fn operator_call_1_class_forward_iterator_int_ptr_(&mut self) {
        (unsafe { test_53() });
    }
    pub unsafe fn operator_call_1_class_cpp17_input_iterator_int_ptr_(&mut self) {
        (unsafe { test_54() });
    }
}
pub unsafe fn test_55() -> bool {
    (unsafe {
        for_each_0 ( <types_type_list_int_ptr__contiguous_iterator_int_ptr___random_access_iterator_int_ptr___bidirectional_iterator_int_ptr___forward_iterator_int_ptr___cpp17_input_iterator_int_ptr__int_ptr__ >::default() , <Test >::default() , )
    });
    {
        let mut lhs: Vec<NonTrivialMod4Comp> = Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 5 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ]);
        let mut rhs: Vec<NonTrivialMod4Comp> = Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ]);
        (unsafe {
            let _lhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut lhs);
            let _rhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut rhs);
            check_43(_lhs, _rhs, 8_usize)
        });
    }
    {
        let mut lhs: Vec<NonTrivialMod4Comp> = Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ]);
        let mut rhs: Vec<NonTrivialMod4Comp> = Vec::from([
            NonTrivialMod4Comp::new({ 1 }),
            NonTrivialMod4Comp::new({ 2 }),
            NonTrivialMod4Comp::new({ 3 }),
            NonTrivialMod4Comp::new({ 4 }),
            NonTrivialMod4Comp::new({ 5 }),
            NonTrivialMod4Comp::new({ 6 }),
            NonTrivialMod4Comp::new({ 7 }),
            NonTrivialMod4Comp::new({ 8 }),
        ]);
        (unsafe {
            let _lhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut lhs);
            let _rhs: Vec<NonTrivialMod4Comp> = std::mem::take(&mut rhs);
            check_43(_lhs, _rhs, 4_usize)
        });
    }
    {
        let mut lhs: Vec<TriviallyEqualityComparable> = Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 5 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ]);
        let mut rhs: Vec<TriviallyEqualityComparable> = Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 5 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ]);
        (unsafe {
            let _lhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut lhs);
            let _rhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut rhs);
            check_44(_lhs, _rhs, 8_usize)
        });
    }
    {
        let mut lhs: Vec<TriviallyEqualityComparable> = Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ]);
        let mut rhs: Vec<TriviallyEqualityComparable> = Vec::from([
            TriviallyEqualityComparable::new({ 1 }),
            TriviallyEqualityComparable::new({ 2 }),
            TriviallyEqualityComparable::new({ 3 }),
            TriviallyEqualityComparable::new({ 4 }),
            TriviallyEqualityComparable::new({ 5 }),
            TriviallyEqualityComparable::new({ 6 }),
            TriviallyEqualityComparable::new({ 7 }),
            TriviallyEqualityComparable::new({ 8 }),
        ]);
        (unsafe {
            let _lhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut lhs);
            let _rhs: Vec<TriviallyEqualityComparable> = std::mem::take(&mut rhs);
            check_44(_lhs, _rhs, 4_usize)
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_55() });
    const _: () = assert!(true, "test()");;
    {
        {
            let mut lhs: Vec<libc::c_char> = (0..(256_usize) as usize)
                .map(|_| (0 as libc::c_char))
                .collect::<Vec<_>>();
            let mut rhs: Vec<libc::c_char> = (0..(256_usize) as usize)
                .map(|_| (0 as libc::c_char))
                .collect::<Vec<_>>();
            let mut i: usize = 0_usize;
            'loop_: while ((i) != (lhs.len())) {
                (*(&mut lhs).as_mut_ptr().add(i as usize)) = (1 as libc::c_char);
                (unsafe { check_45(lhs.clone(), rhs.clone(), i) });
                (*(&mut lhs).as_mut_ptr().add(i as usize)) = (0 as libc::c_char);
                (*(&mut rhs).as_mut_ptr().add(i as usize)) = (1 as libc::c_char);
                (unsafe { check_45(lhs.clone(), rhs.clone(), i) });
                (*(&mut rhs).as_mut_ptr().add(i as usize)) = (0 as libc::c_char);
                i.prefix_inc();
            }
        }
        {
            let mut lhs: Vec<i32> = (0..(256_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
            let mut rhs: Vec<i32> = (0..(256_usize) as usize).map(|_| 0_i32).collect::<Vec<_>>();
            let mut i: usize = 0_usize;
            'loop_: while ((i) != (lhs.len())) {
                (*(&mut lhs).as_mut_ptr().add(i as usize)) = 1;
                (unsafe { check_46(lhs.clone(), rhs.clone(), i) });
                (*(&mut lhs).as_mut_ptr().add(i as usize)) = 0;
                (*(&mut rhs).as_mut_ptr().add(i as usize)) = 1;
                (unsafe { check_46(lhs.clone(), rhs.clone(), i) });
                (*(&mut rhs).as_mut_ptr().add(i as usize)) = 0;
                i.prefix_inc();
            }
        }
    }
    {
        let mut vec_size: usize = 1_usize;
        'loop_: while ((vec_size) != (256_usize)) {
            {
                let mut lhs: Vec<libc::c_char> = (0..(vec_size) as usize)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Vec<_>>();
                let mut rhs: Vec<libc::c_char> = (0..(vec_size) as usize)
                    .map(|_| (0 as libc::c_char))
                    .collect::<Vec<_>>();
                (unsafe {
                    let _lhs: Vec<libc::c_char> = lhs.clone();
                    let _offset: usize = lhs.len();
                    check_45(_lhs, rhs.clone(), _offset)
                });
                (*((&mut lhs).last_mut().unwrap())) = (1 as libc::c_char);
                (unsafe {
                    let _lhs: Vec<libc::c_char> = lhs.clone();
                    let _offset: usize = (lhs.len()).wrapping_sub(1_usize);
                    check_45(_lhs, rhs.clone(), _offset)
                });
                (*((&mut lhs).last_mut().unwrap())) = (0 as libc::c_char);
                (*((&mut rhs).last_mut().unwrap())) = (1 as libc::c_char);
                (unsafe {
                    let _lhs: Vec<libc::c_char> = lhs.clone();
                    let _offset: usize = (lhs.len()).wrapping_sub(1_usize);
                    check_45(_lhs, rhs.clone(), _offset)
                });
                (*((&mut rhs).last_mut().unwrap())) = (0 as libc::c_char);
            }
            {
                let mut lhs: Vec<i32> = (0..(vec_size) as usize).map(|_| 0_i32).collect::<Vec<_>>();
                let mut rhs: Vec<i32> = (0..(vec_size) as usize).map(|_| 0_i32).collect::<Vec<_>>();
                (unsafe {
                    let _lhs: Vec<i32> = lhs.clone();
                    let _offset: usize = lhs.len();
                    check_46(_lhs, rhs.clone(), _offset)
                });
                (*((&mut lhs).last_mut().unwrap())) = 1;
                (unsafe {
                    let _lhs: Vec<i32> = lhs.clone();
                    let _offset: usize = (lhs.len()).wrapping_sub(1_usize);
                    check_46(_lhs, rhs.clone(), _offset)
                });
                (*((&mut lhs).last_mut().unwrap())) = 0;
                (*((&mut rhs).last_mut().unwrap())) = 1;
                (unsafe {
                    let _lhs: Vec<i32> = lhs.clone();
                    let _offset: usize = (lhs.len()).wrapping_sub(1_usize);
                    check_46(_lhs, rhs.clone(), _offset)
                });
                (*((&mut rhs).last_mut().unwrap())) = 0;
            }
            vec_size.prefix_inc();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
