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
pub unsafe fn operator_ne_0(
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
            !(operator_ne_0(
                self as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
                other as *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
impl UnsafeCppAdvance for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_5();
    }
}
impl UnsafeCppIterator<i32> for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe {
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::operator_deref(self) as *mut i32
        }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            cpp17_input_iterator_const_int_ptr__const_int_ptr_::operator_inc_5(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for cpp17_input_iterator_const_int_ptr__const_int_ptr_ {}
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
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
            !(operator_ne_1(
                self as *const forward_iterator_const_int_ptr_,
                other as *const forward_iterator_const_int_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for forward_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for forward_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<i32> for forward_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { forward_iterator_const_int_ptr_::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            forward_iterator_const_int_ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for forward_iterator_const_int_ptr_ {}
pub unsafe fn operator_ne_2(
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
            !(operator_ne_2(
                self as *const bidirectional_iterator_const_int_ptr_,
                other as *const bidirectional_iterator_const_int_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for bidirectional_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for bidirectional_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_6();
    }
}
impl UnsafeCppIterator<i32> for bidirectional_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { bidirectional_iterator_const_int_ptr_::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            bidirectional_iterator_const_int_ptr_::operator_inc_6(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for bidirectional_iterator_const_int_ptr_ {}
pub unsafe fn operator_ne_3(
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
            !(operator_ne_3(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_const_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
}
impl UnsafeCppIterator<i32> for random_access_iterator_const_int_ptr_ {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { random_access_iterator_const_int_ptr_::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            random_access_iterator_const_int_ptr_::operator_inc_7(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for random_access_iterator_const_int_ptr_ {}
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
pub unsafe fn operator_eq_4(
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
            operator_eq_4(
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
pub unsafe fn operator_add_5(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_6(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_5(_i, n)
    });
}
pub unsafe fn iter_swap_7(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_8(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_9(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_10(
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
        unsafe { operator_cmp_10(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_9(self as *const adl_Iterator, other as *const adl_Iterator) }
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
impl UnsafeCppIterator<i32> for adl_Iterator {
    fn iter_deref(&self) -> *mut i32 {
        unsafe { adl_Iterator::operator_deref(self) as *mut i32 }
    }
    fn iter_inc(&mut self) -> &mut Self {
        unsafe {
            adl_Iterator::operator_inc_14(self);
        }
        self
    }
}
impl UnsafeCppInputIterator<i32> for adl_Iterator {}
impl UnsafeCppBidirectionalIterator<i32> for adl_Iterator {
    fn iter_dec(&mut self) -> &mut Self {
        unsafe {
            adl_Iterator::operator_dec_16(self);
        }
        self
    }
}
impl UnsafeCppRandomAccessIterator<i32> for adl_Iterator {
    fn iter_add_assign(&mut self, n: isize) -> Self {
        unsafe {
            adl_Iterator::operator_add_assign(self, n as _);
        }
        self.clone()
    }
    fn iter_distance(&self, from: &Self) -> isize {
        unsafe { (adl_Iterator::operator_sub_11(self, from.clone())) as isize }
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
pub struct add_one {}
impl add_one {
    pub unsafe fn operator_call_1_unsigned_long(&self, mut x: u64) -> u64 {
        return (x).wrapping_add(1_u64);
    }
    pub unsafe fn operator_call_1_unsigned_char(&self, mut x: u8) -> u8 {
        return (((x as i32) + (1)) as u8);
    }
    pub unsafe fn operator_call_1_int(&self, mut x: i32) -> i32 {
        return ((x) + (1));
    }
}
pub unsafe fn test_11(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_12(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_13(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_14(
    mut first: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut last: cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_15(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_16(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_17(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_18(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_19(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_20(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_21(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_22(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_23(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_24(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_25(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_26(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_27(
    mut first: *const i32,
    mut last: *const i32,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_28(
    mut first: *const i32,
    mut last: *const i32,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_29(
    mut first: *const i32,
    mut last: *const i32,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_30(
    mut first: *const i32,
    mut last: *const i32,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_31(
    mut first: *mut i32,
    mut last: *mut i32,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_32(
    mut first: *mut i32,
    mut last: *mut i32,
    mut bop: (),
    mut uop: add_one,
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_33(
    mut first: *mut i32,
    mut last: *mut i32,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_34(
    mut first: *mut i32,
    mut last: *mut i32,
    mut bop: (),
    mut uop: (),
    mut init: i32,
    mut rFirst: *const i32,
    mut rLast: *const i32,
) {
    assert!(
        ((((rLast as usize - rFirst as usize) / ::std::mem::size_of::<i32>()) as i64) <= (5_i64))
    );
    let mut out: [i32; 5] = [0_i32; 5];
    let mut end: *mut i32 = unimplemented!();
    assert!(unimplemented!());
    {
        let mut __first = first;
        let __last = last;
        let mut __out = out.as_mut_ptr();
        while __first != __last {
            *__out.iter_deref() = (*__first.iter_deref());
            __first.iter_inc();
            __out.iter_inc();
        }
        __out
    };
    end = unimplemented!();
    assert!(unimplemented!());
}
pub unsafe fn test_35() {
    let mut ia: [i32; 5] = [1, 3, 5, 7, 9];
    let pResI0: [i32; 5] = [2, 6, 12, 20, 30];
    let mResI0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResN0: [i32; 5] = [-1_i32, -4_i32, -9_i32, -16_i32, -25_i32];
    let mResN0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResI2: [i32; 5] = [4, 8, 14, 22, 32];
    let mResI2: [i32; 5] = [4, 16, 96, 768, 7680];
    let pResN2: [i32; 5] = [1, -2_i32, -7_i32, -14_i32, -23_i32];
    let mResN2: [i32; 5] = [-2_i32, 6, -30_i32, 210, -1890_i32];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResI2) / sizeof(pResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI2) / sizeof(mResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN2) / sizeof(pResN2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN2) / sizeof(mResN2[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (sa)) {
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResI0.as_ptr();
            let _rLast: *const i32 = pResI0.as_ptr().offset((i) as isize);
            test_11(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResI0.as_ptr();
            let _rLast: *const i32 = mResI0.as_ptr().offset((i) as isize);
            test_12(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResN0.as_ptr();
            let _rLast: *const i32 = pResN0.as_ptr().offset((i) as isize);
            test_13(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResN0.as_ptr();
            let _rLast: *const i32 = mResN0.as_ptr().offset((i) as isize);
            test_14(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResI2.as_ptr();
            let _rLast: *const i32 = pResI2.as_ptr().offset((i) as isize);
            test_11(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResI2.as_ptr();
            let _rLast: *const i32 = mResI2.as_ptr().offset((i) as isize);
            test_12(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResN2.as_ptr();
            let _rLast: *const i32 = pResN2.as_ptr().offset((i) as isize);
            test_13(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr()).cast_const()
                });
            let _last: cpp17_input_iterator_const_int_ptr__const_int_ptr_ =
                cpp17_input_iterator_const_int_ptr__const_int_ptr_::new({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResN2.as_ptr();
            let _rLast: *const i32 = mResN2.as_ptr().offset((i) as isize);
            test_14(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub unsafe fn test_36() {
    let mut ia: [i32; 5] = [1, 3, 5, 7, 9];
    let pResI0: [i32; 5] = [2, 6, 12, 20, 30];
    let mResI0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResN0: [i32; 5] = [-1_i32, -4_i32, -9_i32, -16_i32, -25_i32];
    let mResN0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResI2: [i32; 5] = [4, 8, 14, 22, 32];
    let mResI2: [i32; 5] = [4, 16, 96, 768, 7680];
    let pResN2: [i32; 5] = [1, -2_i32, -7_i32, -14_i32, -23_i32];
    let mResN2: [i32; 5] = [-2_i32, 6, -30_i32, 210, -1890_i32];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResI2) / sizeof(pResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI2) / sizeof(mResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN2) / sizeof(pResN2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN2) / sizeof(mResN2[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (sa)) {
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = pResI0.as_ptr();
            let _rLast: *const i32 = pResI0.as_ptr().offset((i) as isize);
            test_15(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = mResI0.as_ptr();
            let _rLast: *const i32 = mResI0.as_ptr().offset((i) as isize);
            test_16(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = pResN0.as_ptr();
            let _rLast: *const i32 = pResN0.as_ptr().offset((i) as isize);
            test_17(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = mResN0.as_ptr();
            let _rLast: *const i32 = mResN0.as_ptr().offset((i) as isize);
            test_18(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = pResI2.as_ptr();
            let _rLast: *const i32 = pResI2.as_ptr().offset((i) as isize);
            test_15(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = mResI2.as_ptr();
            let _rLast: *const i32 = mResI2.as_ptr().offset((i) as isize);
            test_16(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = pResN2.as_ptr();
            let _rLast: *const i32 = pResN2.as_ptr().offset((i) as isize);
            test_17(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: forward_iterator_const_int_ptr_ =
                forward_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: forward_iterator_const_int_ptr_ = forward_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((i) as isize)).cast_const()
            });
            let _rFirst: *const i32 = mResN2.as_ptr();
            let _rLast: *const i32 = mResN2.as_ptr().offset((i) as isize);
            test_18(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub unsafe fn test_37() {
    let mut ia: [i32; 5] = [1, 3, 5, 7, 9];
    let pResI0: [i32; 5] = [2, 6, 12, 20, 30];
    let mResI0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResN0: [i32; 5] = [-1_i32, -4_i32, -9_i32, -16_i32, -25_i32];
    let mResN0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResI2: [i32; 5] = [4, 8, 14, 22, 32];
    let mResI2: [i32; 5] = [4, 16, 96, 768, 7680];
    let pResN2: [i32; 5] = [1, -2_i32, -7_i32, -14_i32, -23_i32];
    let mResN2: [i32; 5] = [-2_i32, 6, -30_i32, 210, -1890_i32];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResI2) / sizeof(pResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI2) / sizeof(mResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN2) / sizeof(pResN2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN2) / sizeof(mResN2[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (sa)) {
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResI0.as_ptr();
            let _rLast: *const i32 = pResI0.as_ptr().offset((i) as isize);
            test_19(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResI0.as_ptr();
            let _rLast: *const i32 = mResI0.as_ptr().offset((i) as isize);
            test_20(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResN0.as_ptr();
            let _rLast: *const i32 = pResN0.as_ptr().offset((i) as isize);
            test_21(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResN0.as_ptr();
            let _rLast: *const i32 = mResN0.as_ptr().offset((i) as isize);
            test_22(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResI2.as_ptr();
            let _rLast: *const i32 = pResI2.as_ptr().offset((i) as isize);
            test_19(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResI2.as_ptr();
            let _rLast: *const i32 = mResI2.as_ptr().offset((i) as isize);
            test_20(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResN2.as_ptr();
            let _rLast: *const i32 = pResN2.as_ptr().offset((i) as isize);
            test_21(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: bidirectional_iterator_const_int_ptr_ =
                bidirectional_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResN2.as_ptr();
            let _rLast: *const i32 = mResN2.as_ptr().offset((i) as isize);
            test_22(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub unsafe fn test_38() {
    let mut ia: [i32; 5] = [1, 3, 5, 7, 9];
    let pResI0: [i32; 5] = [2, 6, 12, 20, 30];
    let mResI0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResN0: [i32; 5] = [-1_i32, -4_i32, -9_i32, -16_i32, -25_i32];
    let mResN0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResI2: [i32; 5] = [4, 8, 14, 22, 32];
    let mResI2: [i32; 5] = [4, 16, 96, 768, 7680];
    let pResN2: [i32; 5] = [1, -2_i32, -7_i32, -14_i32, -23_i32];
    let mResN2: [i32; 5] = [-2_i32, 6, -30_i32, 210, -1890_i32];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResI2) / sizeof(pResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI2) / sizeof(mResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN2) / sizeof(pResN2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN2) / sizeof(mResN2[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (sa)) {
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResI0.as_ptr();
            let _rLast: *const i32 = pResI0.as_ptr().offset((i) as isize);
            test_23(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResI0.as_ptr();
            let _rLast: *const i32 = mResI0.as_ptr().offset((i) as isize);
            test_24(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResN0.as_ptr();
            let _rLast: *const i32 = pResN0.as_ptr().offset((i) as isize);
            test_25(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResN0.as_ptr();
            let _rLast: *const i32 = mResN0.as_ptr().offset((i) as isize);
            test_26(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResI2.as_ptr();
            let _rLast: *const i32 = pResI2.as_ptr().offset((i) as isize);
            test_23(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResI2.as_ptr();
            let _rLast: *const i32 = mResI2.as_ptr().offset((i) as isize);
            test_24(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = pResN2.as_ptr();
            let _rLast: *const i32 = pResN2.as_ptr().offset((i) as isize);
            test_25(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
            let _last: random_access_iterator_const_int_ptr_ =
                random_access_iterator_const_int_ptr_::new_1({
                    (ia.as_mut_ptr().offset((i) as isize)).cast_const()
                });
            let _rFirst: *const i32 = mResN2.as_ptr();
            let _rLast: *const i32 = mResN2.as_ptr().offset((i) as isize);
            test_26(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub unsafe fn test_39() {
    let mut ia: [i32; 5] = [1, 3, 5, 7, 9];
    let pResI0: [i32; 5] = [2, 6, 12, 20, 30];
    let mResI0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResN0: [i32; 5] = [-1_i32, -4_i32, -9_i32, -16_i32, -25_i32];
    let mResN0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResI2: [i32; 5] = [4, 8, 14, 22, 32];
    let mResI2: [i32; 5] = [4, 16, 96, 768, 7680];
    let pResN2: [i32; 5] = [1, -2_i32, -7_i32, -14_i32, -23_i32];
    let mResN2: [i32; 5] = [-2_i32, 6, -30_i32, 210, -1890_i32];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResI2) / sizeof(pResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI2) / sizeof(mResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN2) / sizeof(pResN2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN2) / sizeof(mResN2[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (sa)) {
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResI0.as_ptr();
            let _rLast: *const i32 = pResI0.as_ptr().offset((i) as isize);
            test_27(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResI0.as_ptr();
            let _rLast: *const i32 = mResI0.as_ptr().offset((i) as isize);
            test_28(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResN0.as_ptr();
            let _rLast: *const i32 = pResN0.as_ptr().offset((i) as isize);
            test_29(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResN0.as_ptr();
            let _rLast: *const i32 = mResN0.as_ptr().offset((i) as isize);
            test_30(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResI2.as_ptr();
            let _rLast: *const i32 = pResI2.as_ptr().offset((i) as isize);
            test_27(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResI2.as_ptr();
            let _rLast: *const i32 = mResI2.as_ptr().offset((i) as isize);
            test_28(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResN2.as_ptr();
            let _rLast: *const i32 = pResN2.as_ptr().offset((i) as isize);
            test_29(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *const i32 = ia.as_mut_ptr();
            let _last: *const i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResN2.as_ptr();
            let _rLast: *const i32 = mResN2.as_ptr().offset((i) as isize);
            test_30(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub unsafe fn test_40() {
    let mut ia: [i32; 5] = [1, 3, 5, 7, 9];
    let pResI0: [i32; 5] = [2, 6, 12, 20, 30];
    let mResI0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResN0: [i32; 5] = [-1_i32, -4_i32, -9_i32, -16_i32, -25_i32];
    let mResN0: [i32; 5] = [0, 0, 0, 0, 0];
    let pResI2: [i32; 5] = [4, 8, 14, 22, 32];
    let mResI2: [i32; 5] = [4, 16, 96, 768, 7680];
    let pResN2: [i32; 5] = [1, -2_i32, -7_i32, -14_i32, -23_i32];
    let mResN2: [i32; 5] = [-2_i32, 6, -30_i32, 210, -1890_i32];
    let sa: u32 = (((::std::mem::size_of::<[i32; 5]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    const _: () = assert!(true, "sa == sizeof(pResI0) / sizeof(pResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI0) / sizeof(mResI0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN0) / sizeof(pResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN0) / sizeof(mResN0[0])");;
    const _: () = assert!(true, "sa == sizeof(pResI2) / sizeof(pResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResI2) / sizeof(mResI2[0])");;
    const _: () = assert!(true, "sa == sizeof(pResN2) / sizeof(pResN2[0])");;
    const _: () = assert!(true, "sa == sizeof(mResN2) / sizeof(mResN2[0])");;
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (sa)) {
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResI0.as_ptr();
            let _rLast: *const i32 = pResI0.as_ptr().offset((i) as isize);
            test_31(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResI0.as_ptr();
            let _rLast: *const i32 = mResI0.as_ptr().offset((i) as isize);
            test_32(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResN0.as_ptr();
            let _rLast: *const i32 = pResN0.as_ptr().offset((i) as isize);
            test_33(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResN0.as_ptr();
            let _rLast: *const i32 = mResN0.as_ptr().offset((i) as isize);
            test_34(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                0,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResI2.as_ptr();
            let _rLast: *const i32 = pResI2.as_ptr().offset((i) as isize);
            test_31(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResI2.as_ptr();
            let _rLast: *const i32 = mResI2.as_ptr().offset((i) as isize);
            test_32(
                _first,
                _last,
                unimplemented!(),
                <add_one>::default(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = pResN2.as_ptr();
            let _rLast: *const i32 = pResN2.as_ptr().offset((i) as isize);
            test_33(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        (unsafe {
            let _first: *mut i32 = ia.as_mut_ptr();
            let _last: *mut i32 = ia.as_mut_ptr().offset((i) as isize);
            let _rFirst: *const i32 = mResN2.as_ptr();
            let _rLast: *const i32 = mResN2.as_ptr().offset((i) as isize);
            test_34(
                _first,
                _last,
                unimplemented!(),
                unimplemented!(),
                2,
                _rFirst,
                _rLast,
            )
        });
        i.prefix_inc();
    }
}
pub unsafe fn triangle_41(mut n: usize) -> usize {
    return ((n).wrapping_mul(((n).wrapping_add(1_usize)))).wrapping_div(2_usize);
}
pub unsafe fn basic_tests_42() {
    {
        let mut v: Vec<u64> = std::array::from_fn::<_, 10, _>(|_| 0_u64).to_vec();
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (v.len())) {
            assert!(
                (((*{
                    let __idx = i as usize;
                    let __v: &mut Vec<u64> = &mut v;
                    __v.as_mut_ptr().add(__idx)
                }) as usize)
                    == ((50_usize)
                        .wrapping_add(((i).wrapping_add(1_usize)).wrapping_mul(4_usize))))
            );
            i.prefix_inc();
        }
    }
    {
        let mut v: Vec<u64> = std::array::from_fn::<_, 10, _>(|_| 0_u64).to_vec();
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (v.len())) {
            assert!(
                (((*{
                    let __idx = i as usize;
                    let __v: &mut Vec<u64> = &mut v;
                    __v.as_mut_ptr().add(__idx)
                }) as usize)
                    == ((((30_usize).wrapping_add((unsafe { triangle_41(i,) }))).wrapping_add(i))
                        .wrapping_add(1_usize)))
            );
            i.prefix_inc();
        }
    }
    {
        let mut v: Vec<u64> = std::array::from_fn::<_, 10, _>(|_| 0_u64).to_vec();
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (v.len())) {
            assert!(
                (((*{
                    let __idx = i as usize;
                    let __v: &mut Vec<u64> = &mut v;
                    __v.as_mut_ptr().add(__idx)
                }) as usize)
                    == ((((40_usize)
                        .wrapping_add((unsafe { triangle_41((i).wrapping_add(1_usize),) })))
                    .wrapping_add(i))
                    .wrapping_add(1_usize)))
            );
            i.prefix_inc();
        }
    }
    {
        let mut v: Vec<u64> = std::array::from_fn::<_, 0, _>(|_| 0_u64).to_vec();
        let mut res: Vec<u64> = std::array::from_fn::<_, 0, _>(|_| 0_u64).to_vec();
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut v: Vec<u8> = std::array::from_fn::<_, 10, _>(|_| 0_u8).to_vec();
        unimplemented!();
        let mut res: Vec<u64> = std::array::from_fn::<_, 10, _>(|_| 0_u64).to_vec();
        unimplemented!();
        assert!(((res.len()) == (10_usize)));
        let mut j: usize = 2_usize;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<u64> = &mut res;
                __v.as_mut_ptr().add(__idx)
            }) == (2_u64))
        );
        let mut i: usize = 1_usize;
        'loop_: while ((i) < (res.len())) {
            j = (j).wrapping_mul((i).wrapping_add(2_usize));
            assert!(
                (((*{
                    let __idx = i as usize;
                    let __v: &mut Vec<u64> = &mut res;
                    __v.as_mut_ptr().add(__idx)
                }) as usize)
                    == (j))
            );
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_43() -> bool {
    (unsafe { basic_tests_42() });
    (unsafe { test_35() });
    (unsafe { test_36() });
    (unsafe { test_37() });
    (unsafe { test_38() });
    (unsafe { test_39() });
    (unsafe { test_40() });
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
    (unsafe { test_43() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
