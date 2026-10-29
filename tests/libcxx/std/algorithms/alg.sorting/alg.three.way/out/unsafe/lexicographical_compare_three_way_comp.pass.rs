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
pub unsafe fn operator_eq_0(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_1(
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
            operator_eq_0(
                self as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
                other as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_eq_2(
    x: *const forward_iterator_const_int_ptr_,
    y: *const forward_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_3(
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
            operator_eq_2(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
pub unsafe fn operator_eq_8(
    x: *const contiguous_iterator_const_int_ptr_,
    y: *const contiguous_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_9(
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
    pub unsafe fn operator_deref(&self) -> *mut i32 {
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
            operator_eq_8(
                self as *const contiguous_iterator_const_int_ptr_,
                other as *const contiguous_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for contiguous_iterator_const_int_ptr_ {}
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
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct util_iterator_error {}
impl util_iterator_error {
    pub unsafe fn new(mut what: *const libc::c_char) -> Self {
        let mut this = Self {};
        this
    }
}
pub static mut compare_last_digit_strong_17: std::cell::LazyCell<
    FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
> = std::cell::LazyCell::new(|| unsafe {
    FnPtr::<fn(i32, i32) -> std::cmp::Ordering>::new(
        |mut a: i32, mut b: i32| -> std::cmp::Ordering {
            unsafe {
                return std::cmp::Ord::cmp(&((a) % (10)), &((b) % (10)));
            }
        },
    )
});
pub static mut compare_last_digit_weak_18: std::cell::LazyCell<
    FnPtr<fn(i32, i32) -> std_weak_ordering>,
> = std::cell::LazyCell::new(|| unsafe {
    FnPtr::<fn(i32, i32) -> std_weak_ordering>::new(|mut a: i32, mut b: i32| -> std_weak_ordering {
        unsafe {
            return (unsafe { std::cmp::Ord::cmp(&((a) % (10)), &((b) % (10)))() });
        }
    })
});
pub static mut compare_last_digit_partial_19: std::cell::LazyCell<
    FnPtr<fn(i32, i32) -> std_partial_ordering>,
> = std::cell::LazyCell::new(|| unsafe {
    FnPtr::<fn(i32, i32) -> std_partial_ordering>::new(
        |mut a: i32, mut b: i32| -> std_partial_ordering {
            unsafe {
                if ((a) == (<i32>::MIN)) || ((b) == (<i32>::MIN)) {
                    return (*std::cell::LazyCell::force_mut(&mut *&raw mut unordered_20));
                }
                return (unsafe { std::cmp::Ord::cmp(&((a) % (10)), &((b) % (10)))() });
            }
        },
    )
});
pub static mut compare_int_result_21: std::cell::LazyCell<FnPtr<fn(i32, i32) -> i32>> =
    std::cell::LazyCell::new(|| unsafe {
        FnPtr::<fn(i32, i32) -> i32>::new(|mut a: i32, mut b: i32| -> i32 {
            unsafe {
                return (((a) % (10)) - ((b) % (10)));
            }
        })
    });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StructWithoutCallOperator {}
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_strong)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_weak)>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_last_digit_partial)>"
);
const _: () = assert!(
    !(false),
    "!has_lexicographical_compare<StructWithoutCallOperator>"
);
const _: () = assert!(
    true,
    "has_lexicographical_compare<decltype(compare_int_result)>"
);
pub unsafe fn test_lexicographical_compare_22(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_23(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_24(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_23(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_25(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_23(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_26(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_23(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_27(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_23(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_28(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_23(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_29(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_30(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_31(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_30(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_32(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_30(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_33(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_30(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_34(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_30(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_35(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_30(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_36(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_37(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_38(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_37(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_39(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_37(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_40(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_37(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_41(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_37(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_42(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_37(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_43(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_44(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_45(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_44(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_46(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_44(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_47(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_44(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_48(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_44(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_49(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_44(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_50(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_51(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_52(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_51(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_53(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_51(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_54(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_51(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_55(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_51(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_56(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_51(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_57(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_58(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_59(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_58(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_60(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_58(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_61(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_58(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_62(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_58(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_63(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_58(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_64(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_65(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_66(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_65(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_67(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_65(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_68(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_65(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_69(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_65(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_70(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *mut i32 = a.as_mut_ptr();
        let ___last1: *mut i32 = a.as_mut_ptr().offset((a.len()) as isize);
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_65(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_71(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_72(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_73(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_72(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_74(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_72(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_75(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_72(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_76(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_72(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_77(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_72(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_78(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_79(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_80(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_79(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_81(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_79(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_82(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_79(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_83(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_79(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_84(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_79(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_85(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_86(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_87(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_86(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_88(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_86(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_89(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_86(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_90(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_86(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_91(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_86(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_92(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_93(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_94(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_93(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_95(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_93(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_96(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_93(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_97(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_93(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_98(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_93(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_99(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_100(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_101(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_100(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_102(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_100(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_103(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_100(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_104(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_100(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_105(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_100(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_106(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_107(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_108(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_107(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_109(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_107(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_110(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_107(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_111(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_107(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_112(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_107(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_113(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_114(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_115(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_114(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_116(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_114(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_117(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_114(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_118(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_114(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_119(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_114(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_120(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_121(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_122(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_121(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_123(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_121(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_124(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_121(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_125(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_121(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_126(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_121(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_127(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_128(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_129(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_128(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_130(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_128(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_131(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_128(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_132(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_128(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_133(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_128(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_134(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_135(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_136(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_135(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_137(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_135(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_138(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_135(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_139(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_135(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_140(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_135(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_141(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_142(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_143(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_142(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_144(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_142(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_145(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_142(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_146(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_142(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_147(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_142(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_148(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_149(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_150(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_149(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_151(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_149(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_152(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_149(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_153(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_149(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_154(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_149(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_155(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_156(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_157(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_156(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_158(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_156(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_159(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_156(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_160(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_156(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_161(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_156(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_162(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_163(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_164(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_163(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_165(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_163(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_166(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_163(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_167(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_163(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_168(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr()).cast_const()
            });
        let ___last1: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_163(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_169(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_170(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_171(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_170(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_172(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_170(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_173(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_170(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_174(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_170(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_175(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_170(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_176(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_177(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_178(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_177(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_179(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_177(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_180(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_177(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_181(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_177(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_182(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_177(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_183(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_184(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_185(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_184(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_186(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_184(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_187(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_184(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_188(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_184(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_189(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_184(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_190(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_191(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_192(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_191(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_193(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_191(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_194(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_191(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_195(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_191(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_196(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_191(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_197(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_198(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_199(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_198(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_200(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_198(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_201(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_198(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_202(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_198(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_203(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_198(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_204(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_205(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_206(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_205(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_207(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_205(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_208(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_205(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_209(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_205(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_210(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_205(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_211(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_212(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_213(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_212(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_214(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_212(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_215(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_212(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_216(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_212(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_217(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
        });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_212(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_218(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_219(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_220(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_219(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_221(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_219(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_222(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_219(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_223(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_219(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_224(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_219(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_225(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_226(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_227(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_226(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_228(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_226(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_229(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_226(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_230(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_226(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_231(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_226(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_232(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_233(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_234(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_233(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_235(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_233(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_236(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_233(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_237(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_233(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_238(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_233(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_239(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_240(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_241(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_240(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_242(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_240(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_243(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_240(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_244(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_240(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_245(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_240(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_246(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_247(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_248(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_247(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_249(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_247(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_250(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_247(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_251(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_247(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_252(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_247(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_253(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_254(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_255(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_254(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_256(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_254(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_257(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_254(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_258(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_254(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_259(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_254(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_260(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_261(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_262(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_261(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_263(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_261(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_264(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_261(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_265(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_261(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_266(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_261(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_267(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_268(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_269(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_268(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_270(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_268(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_271(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_268(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_272(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_268(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_273(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_268(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_274(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_275(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_276(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_275(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_277(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_275(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_278(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_275(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_279(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_275(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_280(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_275(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_281(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_282(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_283(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_282(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_284(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_282(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_285(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_282(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_286(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_282(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_287(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_282(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_288(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_289(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_290(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_289(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_291(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_289(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_292(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_289(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_293(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_289(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_294(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_289(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_295(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_296(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_297(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_296(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_298(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_296(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_299(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_296(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_300(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_296(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_301(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_296(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_302(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_303(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_304(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_303(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_305(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_303(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_306(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_303(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_307(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_303(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_308(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_303(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_309(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_310(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_311(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_310(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_312(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_310(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_313(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_310(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_314(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_310(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_315(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_310(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_316(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_317(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_318(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_317(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_319(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_317(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_320(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_317(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_321(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_317(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_322(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *mut i32 = b.as_mut_ptr();
        let ___last2: *mut i32 = b.as_mut_ptr().offset((b.len()) as isize);
        lexicographical_compare_three_way_317(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_323(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_324(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_325(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_324(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_326(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_324(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_327(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_324(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_328(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_324(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_329(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_324(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_330(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_331(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_332(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_331(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_333(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_331(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_334(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_331(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_335(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_331(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_336(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr()).cast_const()
            });
        let ___last2: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_331(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_337(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_338(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_339(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_338(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_340(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_338(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_341(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_338(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_342(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_338(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_343(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
            (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
        });
        lexicographical_compare_three_way_338(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_344(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_345(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_346(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_345(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_347(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_345(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_348(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_345(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_349(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_345(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_350(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_345(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_351(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_352(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_353(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_352(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_354(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_352(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_355(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_352(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_356(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_352(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_357(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_352(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_358(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_359(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_360(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_359(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_361(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_359(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_362(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_359(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_363(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_359(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_364(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (a.as_mut_ptr()).cast_const() });
        let ___last1: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (a.as_mut_ptr().offset((a.len()) as isize)).cast_const()
            });
        let ___first2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({ (b.as_mut_ptr()).cast_const() });
        let ___last2: contiguous_iterator_const_int_ptr_ =
            contiguous_iterator_const_int_ptr_::new_1({
                (b.as_mut_ptr().offset((b.len()) as isize)).cast_const()
            });
        lexicographical_compare_three_way_359(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_365(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_79(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_366(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_367(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_lexicographical_compare_368(
    mut a: Vec<i32>,
    mut b: Vec<i32>,
    mut comp: FnPtr<fn(i32, i32) -> std::cmp::Ordering>,
    mut expected: std::cmp::Ordering,
) {
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: *const i32 = (a.as_mut_ptr()).cast_const();
        let ___last1: *const i32 = (a.as_mut_ptr().offset((a.len()) as isize)).cast_const();
        let ___first2: *const i32 = (b.as_mut_ptr()).cast_const();
        let ___last2: *const i32 = (b.as_mut_ptr().offset((b.len()) as isize)).cast_const();
        lexicographical_compare_three_way_367(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            comp.copy_from(),
        )
    });
    assert!(expected == result);
}
pub unsafe fn test_given_iterator_types_369() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_22(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_24(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_25(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_26(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_26(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_26(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_26(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_27(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_28(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_26(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_370() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_29(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_31(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_32(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_34(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_35(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_33(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_371() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_36(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_38(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_39(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_40(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_40(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_40(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_40(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_41(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_42(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_40(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_372() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_43(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_45(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_46(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_47(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_47(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_47(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_47(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_48(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_49(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_47(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_373() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_50(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_52(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_53(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_55(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_56(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_54(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_374() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_57(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_59(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_60(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_61(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_61(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_61(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_61(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_62(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_63(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_61(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_375() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_64(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_66(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_67(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_68(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_68(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_68(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_68(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_69(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_70(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_68(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_376() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_71(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_73(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_74(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_76(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_77(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_75(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_377() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_78(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_80(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_81(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_82(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_82(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_82(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_82(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_83(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_84(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_82(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_378() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_85(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_87(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_88(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_89(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_89(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_89(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_89(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_90(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_91(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_89(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_379() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_92(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_94(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_95(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_97(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_98(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_96(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_380() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_99(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_101(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_102(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_103(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_103(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_103(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_103(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_104(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_105(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_103(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_381() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_106(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_108(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_109(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_110(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_110(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_110(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_110(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_111(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_112(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_110(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_382() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_113(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_115(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_116(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_118(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_119(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_117(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_383() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_120(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_122(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_123(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_124(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_124(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_124(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_124(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_125(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_126(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_124(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_384() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_127(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_129(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_130(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_131(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_131(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_131(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_131(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_132(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_133(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_131(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_385() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_134(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_136(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_137(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_139(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_140(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_138(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_386() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_141(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_143(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_144(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_145(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_145(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_145(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_145(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_146(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_147(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_145(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_387() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_148(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_150(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_151(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_152(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_152(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_152(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_152(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_153(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_154(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_152(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_388() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_155(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_157(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_158(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_160(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_161(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_159(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_389() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_162(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_164(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_165(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_166(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_166(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_166(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_166(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_167(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_168(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_166(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_390() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_169(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_171(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_172(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_173(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_173(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_173(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_173(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_174(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_175(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_173(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_391() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_176(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_178(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_179(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_181(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_182(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_180(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_392() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_183(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_185(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_186(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_187(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_187(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_187(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_187(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_188(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_189(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_187(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_393() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_190(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_192(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_193(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_194(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_194(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_194(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_194(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_195(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_196(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_194(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_394() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_197(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_199(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_200(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_202(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_203(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_201(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_395() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_204(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_206(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_207(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_208(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_208(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_208(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_208(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_209(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_210(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_208(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_396() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_211(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_213(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_214(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_215(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_215(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_215(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_215(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_216(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_217(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_215(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_397() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_218(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_220(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_221(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_223(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_224(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_222(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_398() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_225(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_227(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_228(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_229(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_229(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_229(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_229(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_230(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_231(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_229(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_399() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_232(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_234(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_235(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_236(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_236(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_236(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_236(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_237(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_238(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_236(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_400() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_239(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_241(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_242(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_244(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_245(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_243(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_401() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_246(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_248(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_249(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_250(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_250(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_250(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_250(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_251(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_252(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_250(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_402() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_253(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_255(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_256(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_257(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_257(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_257(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_257(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_258(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_259(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_257(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_403() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_260(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_262(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_263(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_265(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_266(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_264(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_404() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_267(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_269(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_270(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_271(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_271(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_271(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_271(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_272(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_273(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_271(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_405() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_274(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_276(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_277(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_278(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_278(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_278(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_278(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_279(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_280(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_278(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_406() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_281(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_283(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_284(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_286(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_287(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_285(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_407() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_288(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_290(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_291(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_292(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_292(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_292(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_292(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_293(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_294(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_292(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_408() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_295(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_297(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_298(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_299(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_299(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_299(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_299(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_300(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_301(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_299(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_409() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_302(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_304(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_305(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_307(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_308(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_306(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_410() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_309(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_311(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_312(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_313(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_313(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_313(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_313(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_314(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_315(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_313(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_411() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_316(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_318(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_319(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_320(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_320(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_320(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_320(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_321(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_322(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_320(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_412() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_323(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_325(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_326(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_327(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_327(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_327(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_327(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_328(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_329(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_327(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_413() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_330(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_332(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_333(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_334(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_334(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_334(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_334(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_335(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_336(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_334(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_414() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_337(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_339(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_340(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_341(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_341(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_341(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_341(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_342(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_343(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_341(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_415() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_344(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_346(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_347(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_348(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_348(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_348(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_348(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_349(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_350(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_348(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_416() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_351(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_353(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_354(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_355(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_355(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_355(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_355(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_356(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_357(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_355(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_given_iterator_types_417() {
    let mut cmp: FnPtr<fn(i32, i32) -> std::cmp::Ordering> =
        (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17)).copy_from();
    (unsafe {
        test_lexicographical_compare_358(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_360(
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_361(
            Vec::from([0, 1]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_362(
            Vec::from([0, 1]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
    (unsafe {
        test_lexicographical_compare_362(
            Vec::from([0, 1]),
            Vec::from([0, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_362(
            Vec::from([0, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_362(
            Vec::from([0, 2]),
            Vec::from([1, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_363(
            Vec::from([0, 1]),
            Vec::from([0, 1, 2]),
            cmp.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_364(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1]),
            cmp.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    (unsafe {
        test_lexicographical_compare_362(
            Vec::from([10, 21]),
            Vec::from([10, 31]),
            cmp.copy_from(),
            std::cmp::Ordering::Equal,
        )
    });
}
pub unsafe fn test_iterator_types1_418() {
    (unsafe { test_given_iterator_types_369() });
    (unsafe { test_given_iterator_types_370() });
    (unsafe { test_given_iterator_types_371() });
    (unsafe { test_given_iterator_types_372() });
    (unsafe { test_given_iterator_types_373() });
    (unsafe { test_given_iterator_types_374() });
    (unsafe { test_given_iterator_types_375() });
}
pub unsafe fn test_iterator_types1_419() {
    (unsafe { test_given_iterator_types_376() });
    (unsafe { test_given_iterator_types_377() });
    (unsafe { test_given_iterator_types_378() });
    (unsafe { test_given_iterator_types_379() });
    (unsafe { test_given_iterator_types_380() });
    (unsafe { test_given_iterator_types_381() });
    (unsafe { test_given_iterator_types_382() });
}
pub unsafe fn test_iterator_types1_420() {
    (unsafe { test_given_iterator_types_383() });
    (unsafe { test_given_iterator_types_384() });
    (unsafe { test_given_iterator_types_385() });
    (unsafe { test_given_iterator_types_386() });
    (unsafe { test_given_iterator_types_387() });
    (unsafe { test_given_iterator_types_388() });
    (unsafe { test_given_iterator_types_389() });
}
pub unsafe fn test_iterator_types1_421() {
    (unsafe { test_given_iterator_types_390() });
    (unsafe { test_given_iterator_types_391() });
    (unsafe { test_given_iterator_types_392() });
    (unsafe { test_given_iterator_types_393() });
    (unsafe { test_given_iterator_types_394() });
    (unsafe { test_given_iterator_types_395() });
    (unsafe { test_given_iterator_types_396() });
}
pub unsafe fn test_iterator_types1_422() {
    (unsafe { test_given_iterator_types_397() });
    (unsafe { test_given_iterator_types_398() });
    (unsafe { test_given_iterator_types_399() });
    (unsafe { test_given_iterator_types_400() });
    (unsafe { test_given_iterator_types_401() });
    (unsafe { test_given_iterator_types_402() });
    (unsafe { test_given_iterator_types_403() });
}
pub unsafe fn test_iterator_types1_423() {
    (unsafe { test_given_iterator_types_404() });
    (unsafe { test_given_iterator_types_405() });
    (unsafe { test_given_iterator_types_406() });
    (unsafe { test_given_iterator_types_407() });
    (unsafe { test_given_iterator_types_408() });
    (unsafe { test_given_iterator_types_409() });
    (unsafe { test_given_iterator_types_410() });
}
pub unsafe fn test_iterator_types1_424() {
    (unsafe { test_given_iterator_types_411() });
    (unsafe { test_given_iterator_types_412() });
    (unsafe { test_given_iterator_types_413() });
    (unsafe { test_given_iterator_types_414() });
    (unsafe { test_given_iterator_types_415() });
    (unsafe { test_given_iterator_types_416() });
    (unsafe { test_given_iterator_types_417() });
}
pub unsafe fn test_iterator_types_425() {
    (unsafe { test_iterator_types1_418() });
    (unsafe { test_iterator_types1_419() });
    (unsafe { test_iterator_types1_420() });
    (unsafe { test_iterator_types1_421() });
    (unsafe { test_iterator_types1_422() });
    (unsafe { test_iterator_types1_423() });
    (unsafe { test_iterator_types1_424() });
}
pub unsafe fn test_comparison_categories_426() {
    (unsafe {
        test_lexicographical_compare_427(
            Vec::from([0, 1]),
            Vec::from([10, 11]),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_weak_18))
                .copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut equivalent_428)),
        )
    });
    (unsafe {
        test_lexicographical_compare_429(
            Vec::from([0, 1]),
            Vec::from([20, 11]),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_partial_19))
                .copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut equivalent_430)),
        )
    });
    (unsafe {
        test_lexicographical_compare_365(
            Vec::from([0]),
            Vec::from([0, 1]),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17))
                .copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    (unsafe {
        test_lexicographical_compare_431(
            Vec::from([0]),
            Vec::from([0, 1]),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_weak_18))
                .copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut less_432)),
        )
    });
    (unsafe {
        test_lexicographical_compare_433(
            Vec::from([0]),
            Vec::from([0, 1]),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_partial_19))
                .copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut less_434)),
        )
    });
    (unsafe {
        test_lexicographical_compare_435(
            Vec::from([<i32>::MIN, 1]),
            Vec::from([0, 1, 2]),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_partial_19))
                .copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut unordered_20)),
        )
    });
}
pub unsafe fn test_comparator_invocation_count_436() {
    let mut compare_invocation_count: i32 = 0;
    let mut compare_last_digit_counting: FnPtr<fn(i32, i32) -> std::cmp::Ordering> = lambda_unsafe!(
        {
            let compare_invocation_count: *mut i32 = &mut compare_invocation_count;
        },
        |mut a: i32, mut b: i32| -> std::cmp::Ordering {
            (*compare_invocation_count).prefix_inc();
            return std::cmp::Ord::cmp(&((a) % (10)), &((b) % (10)));
        }
    );
    compare_invocation_count = 0;
    (unsafe {
        test_lexicographical_compare_366(
            Vec::from([0, 1, 2, 3]),
            unsafe { std::mem::zeroed::<Vec<i32>>() },
            compare_last_digit_counting.copy_from(),
            std::cmp::Ordering::Greater,
        )
    });
    assert!(((compare_invocation_count) == (0)));
    (unsafe {
        test_lexicographical_compare_368(
            Vec::from([0, 1, 2]),
            Vec::from([0, 1, 2, 3]),
            compare_last_digit_counting.copy_from(),
            std::cmp::Ordering::Less,
        )
    });
    assert!(((compare_invocation_count) <= (6)));
}
pub unsafe fn test_proxy_iterators_437() {
    let mut vec_: Vec<bool> = (0..(10_usize) as usize)
        .map(|_| true)
        .collect::<Vec<bool>>();
    let mut result: std::cmp::Ordering = (unsafe {
        let ___first1: UnsafeRandomAccessIterator<bool> = vec_.begin();
        let ___last1: UnsafeRandomAccessIterator<bool> = vec_.end();
        let ___first2: UnsafeRandomAccessIterator<bool> = vec_.begin();
        let ___last2: UnsafeRandomAccessIterator<bool> = vec_.end();
        lexicographical_compare_three_way_438(
            ___first1,
            ___last1,
            ___first2,
            ___last2,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut compare_last_digit_strong_17))
                .copy_from(),
        )
    });
    assert!(result == std::cmp::Ordering::Equal);
}
pub unsafe fn test_439() -> bool {
    (unsafe { test_iterator_types_425() });
    (unsafe { test_comparison_categories_426() });
    (unsafe { test_comparator_invocation_count_436() });
    (unsafe { test_proxy_iterators_437() });
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
    (unsafe { test_439() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const compare_last_digit_strong_17);
    std::cell::LazyCell::force(&*&raw const compare_last_digit_weak_18);
    std::cell::LazyCell::force(&*&raw const compare_last_digit_partial_19);
    std::cell::LazyCell::force(&*&raw const compare_int_result_21);
}
