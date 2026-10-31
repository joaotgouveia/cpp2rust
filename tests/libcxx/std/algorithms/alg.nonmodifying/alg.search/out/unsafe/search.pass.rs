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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_0(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_1(
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
            operator_eq_0(
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
pub unsafe fn operator_eq_2(
    x: *const forward_iterator_User_S_ptr_,
    y: *const forward_iterator_User_S_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_3(
    x: *const forward_iterator_User_S_ptr_,
    y: *const forward_iterator_User_S_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_User_S_ptr_ {
    it_: *mut User_S,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_User_S_ptr_ {
    pub unsafe fn new_1(mut it: *mut User_S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut User_S {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_6(&mut self) -> *mut forward_iterator_User_S_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut forward_iterator_User_S_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut forward_iterator_User_S_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut forward_iterator_User_S_ptr_,
    ) -> *mut forward_iterator_User_S_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut forward_iterator_User_S_ptr_));
    }
}
impl std::cmp::PartialEq for forward_iterator_User_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_2(
                self as *const forward_iterator_User_S_ptr_,
                other as *const forward_iterator_User_S_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_User_S_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_User_S_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
pub unsafe fn operator_eq_4(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_5(
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
            operator_eq_4(
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
pub unsafe fn operator_eq_6(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_7(
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
            operator_eq_6(
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
pub unsafe fn operator_eq_8(
    x: *const random_access_iterator_User_S_ptr_,
    y: *const random_access_iterator_User_S_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_9(
    x: *const random_access_iterator_User_S_ptr_,
    y: *const random_access_iterator_User_S_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_User_S_ptr_ {
    it_: *mut User_S,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_User_S_ptr_ {
    pub unsafe fn new_1(mut it: *mut User_S) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut User_S {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_User_S_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_User_S_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_User_S_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_User_S_ptr_,
    ) -> *mut random_access_iterator_User_S_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_User_S_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_User_S_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_8(
                self as *const random_access_iterator_User_S_ptr_,
                other as *const random_access_iterator_User_S_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_User_S_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_User_S_ptr_ {
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
pub unsafe fn operator_eq_10(
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
            operator_eq_10(
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
pub unsafe fn operator_add_11(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_12(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_11(_i, n)
    });
}
pub unsafe fn iter_swap_13(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_14(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_15(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_16(
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
        unsafe { operator_cmp_16(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_15(self as *const adl_Iterator, other as *const adl_Iterator) }
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MySearcherC {}
impl MySearcherC {
    pub unsafe fn operator_call_1_int_ptr(
        &self,
        mut b: *mut i32,
        mut e: *mut i32,
    ) -> (*mut i32, *mut i32) {
        return (b, e);
    }
}
pub unsafe fn test_constexpr_17() -> bool {
    let mut ia: [i32; 4] = [0, 1, 2, 3];
    let mut ib: [i32; 4] = [0, 1, 5, 3];
    let mut ic: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    return (((unimplemented!()) == (ic.as_mut_ptr().offset((3) as isize)))
        && ((unimplemented!()) == (ic.as_ptr().add(ic.len()))))
        && ((unimplemented!()) == (unimplemented!()));
}
pub static mut searcher_called_18: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MySearcher {}
impl MySearcher {
    pub unsafe fn operator_call_1_int_ptr(
        &self,
        mut b: *mut i32,
        mut e: *mut i32,
    ) -> (*mut i32, *mut i32) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut searcher_called_18)).prefix_inc();
        return (b, e);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct User_S {
    pub x_: i32,
}
impl User_S {
    pub unsafe fn new(mut x: i32) -> Self {
        let mut this = Self { x_: x };
        this
    }
}
impl std::cmp::PartialEq for User_S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_19(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for User_S {}
pub unsafe fn operator_eq_19(mut lhs: User_S, mut rhs: User_S) -> bool {
    return ((lhs.x_) == (rhs.x_));
}
pub unsafe fn test_20() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((4) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((8) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ih.as_mut_ptr().offset((3) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ij.as_mut_ptr().offset((6) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_21() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((4) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((8) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ih.as_mut_ptr().offset((3) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ij.as_mut_ptr().offset((6) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_22() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((4) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((8) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ih.as_mut_ptr().offset((3) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: forward_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ij.as_mut_ptr().offset((6) as isize)).cast_const()
            });
            operator_eq_0(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_23() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((3) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((4) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((8) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ih.as_mut_ptr().offset((3) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ij.as_mut_ptr().offset((6) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_24() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((3) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((4) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((8) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ih.as_mut_ptr().offset((3) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ij.as_mut_ptr().offset((6) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_25() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((3) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((4) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((8) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ih.as_mut_ptr().offset((3) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: bidirectional_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ij.as_mut_ptr().offset((6) as isize)).cast_const()
                });
            operator_eq_4(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_26() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((3) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((4) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((8) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ih.as_mut_ptr().offset((3) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ij.as_mut_ptr().offset((6) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_27() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((3) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((4) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((8) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ih.as_mut_ptr().offset((3) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ij.as_mut_ptr().offset((6) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn test_28() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((2) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((3) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr()
                        .offset((sa) as isize)
                        .offset(-((1) as isize)))
                    .cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut id: [i32; 2] = [1, 2];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((1) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ie: [i32; 3] = [1, 2, 3];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((4) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ib.as_mut_ptr().offset((8) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ih.as_mut_ptr().offset((3) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
    let mut ij: [i32; 16] = [0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sj: u32 = (((::std::mem::size_of::<[i32; 16]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ik: [i32; 10] = [0, 0, 0, 0, 1, 1, 1, 1, 0, 0];
    let sk: u32 = (((::std::mem::size_of::<[i32; 10]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_const_int_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ij.as_mut_ptr().offset((6) as isize)).cast_const()
                });
            operator_eq_6(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn adl_test_29() {
    let mut ua: [User_S; 1] = [User_S::new({ 1 })];
    assert!(
        (unsafe {
            let mut _x: forward_iterator_User_S_ptr_ = unimplemented!();
            let mut _y: forward_iterator_User_S_ptr_ =
                forward_iterator_User_S_ptr_::new_1({ ua.as_mut_ptr() });
            operator_eq_2(&mut _x, &mut _y)
        })
    );
}
pub unsafe fn adl_test_30() {
    let mut ua: [User_S; 1] = [User_S::new({ 1 })];
    assert!(
        (unsafe {
            let mut _x: random_access_iterator_User_S_ptr_ = unimplemented!();
            let mut _y: random_access_iterator_User_S_ptr_ =
                random_access_iterator_User_S_ptr_::new_1({ ua.as_mut_ptr() });
            operator_eq_8(&mut _x, &mut _y)
        })
    );
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
    (unsafe { test_20() });
    (unsafe { test_21() });
    (unsafe { test_22() });
    (unsafe { test_23() });
    (unsafe { test_24() });
    (unsafe { test_25() });
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { adl_test_29() });
    (unsafe { adl_test_30() });
    {
        const _: () = assert!(
            true,
            "(std::is_same<RI, decltype(std::search(RI(), RI(), MySearcher()))>::value)"
        );;
        let mut it: *mut i32 = std::ptr::null_mut();
        assert!(((it) == (unimplemented!())));
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut searcher_called_18)) == (1)));
    }
    const _: () = assert!(true, "test_constexpr()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const searcher_called_18);
}
