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
pub unsafe fn base_2(i: *const forward_iterator_const_int_ptr_) -> *const i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct forward_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl forward_iterator_const_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *const i32) -> Self {
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
impl Default for forward_iterator_const_int_ptr_ {
    fn default() -> Self {
        unsafe { forward_iterator_const_int_ptr_::new_1() }
    }
}
pub unsafe fn operator_eq_3(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
pub unsafe fn operator_ne_4(
    x: *const bidirectional_iterator_const_int_ptr_,
    y: *const bidirectional_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) != ((*y).it_));
}
pub unsafe fn base_5(i: *const bidirectional_iterator_const_int_ptr_) -> *const i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct bidirectional_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl bidirectional_iterator_const_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *const i32) -> Self {
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
            operator_eq_3(
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
impl Default for bidirectional_iterator_const_int_ptr_ {
    fn default() -> Self {
        unsafe { bidirectional_iterator_const_int_ptr_::new_1() }
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
pub unsafe fn base_8(i: *const random_access_iterator_const_int_ptr_) -> *const i32 {
    return (*i).it_;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct random_access_iterator_const_int_ptr_ {
    it_: *const i32,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_int_ptr_ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            it_: std::ptr::null(),
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn new_2(mut it: *const i32) -> Self {
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
impl Default for random_access_iterator_const_int_ptr_ {
    fn default() -> Self {
        unsafe { random_access_iterator_const_int_ptr_::new_1() }
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
pub unsafe fn base_9(mut i: *const i32) -> *const i32 {
    return i;
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
pub static mut randomness_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn test_18(
    mut first: forward_iterator_const_int_ptr_,
    mut last: forward_iterator_const_int_ptr_,
) {
    let mut comp: () = unimplemented!();
    let mut p: (
        forward_iterator_const_int_ptr_,
        forward_iterator_const_int_ptr_,
    ) = unimplemented!();
    if (unsafe {
        let _x: *const forward_iterator_const_int_ptr_ = &first;
        operator_ne_1(_x, &last)
    }) {
        let mut j: forward_iterator_const_int_ptr_ = first;
        'loop_: while (unsafe {
            let _x: *const forward_iterator_const_int_ptr_ = &j;
            operator_ne_1(_x, &last)
        }) {
            assert!(!(unimplemented!()));
            assert!(!(unimplemented!()));
            (unsafe { forward_iterator_const_int_ptr_::operator_inc_6(&mut j) });
        }
    } else {
        assert!(
            (unsafe {
                let _x: *const forward_iterator_const_int_ptr_ = &p.0;
                operator_eq_0(_x, &last)
            })
        );
        assert!(
            (unsafe {
                let _x: *const forward_iterator_const_int_ptr_ = &p.1;
                operator_eq_0(_x, &last)
            })
        );
    }
}
pub unsafe fn test_19(
    mut first: bidirectional_iterator_const_int_ptr_,
    mut last: bidirectional_iterator_const_int_ptr_,
) {
    let mut comp: () = unimplemented!();
    let mut p: (
        bidirectional_iterator_const_int_ptr_,
        bidirectional_iterator_const_int_ptr_,
    ) = unimplemented!();
    if (unsafe {
        let _x: *const bidirectional_iterator_const_int_ptr_ = &first;
        operator_ne_4(_x, &last)
    }) {
        let mut j: bidirectional_iterator_const_int_ptr_ = first;
        'loop_: while (unsafe {
            let _x: *const bidirectional_iterator_const_int_ptr_ = &j;
            operator_ne_4(_x, &last)
        }) {
            assert!(!(unimplemented!()));
            assert!(!(unimplemented!()));
            (unsafe { bidirectional_iterator_const_int_ptr_::operator_inc_6(&mut j) });
        }
    } else {
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_const_int_ptr_ = &p.0;
                operator_eq_3(_x, &last)
            })
        );
        assert!(
            (unsafe {
                let _x: *const bidirectional_iterator_const_int_ptr_ = &p.1;
                operator_eq_3(_x, &last)
            })
        );
    }
}
pub unsafe fn test_20(
    mut first: random_access_iterator_const_int_ptr_,
    mut last: random_access_iterator_const_int_ptr_,
) {
    let mut comp: () = unimplemented!();
    let mut p: (
        random_access_iterator_const_int_ptr_,
        random_access_iterator_const_int_ptr_,
    ) = unimplemented!();
    if (unsafe {
        let _x: *const random_access_iterator_const_int_ptr_ = &first;
        operator_ne_7(_x, &last)
    }) {
        let mut j: random_access_iterator_const_int_ptr_ = first;
        'loop_: while (unsafe {
            let _x: *const random_access_iterator_const_int_ptr_ = &j;
            operator_ne_7(_x, &last)
        }) {
            assert!(!(unimplemented!()));
            assert!(!(unimplemented!()));
            (unsafe { random_access_iterator_const_int_ptr_::operator_inc_7(&mut j) });
        }
    } else {
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_const_int_ptr_ = &p.0;
                operator_eq_6(_x, &last)
            })
        );
        assert!(
            (unsafe {
                let _x: *const random_access_iterator_const_int_ptr_ = &p.1;
                operator_eq_6(_x, &last)
            })
        );
    }
}
pub unsafe fn test_21(mut first: *const i32, mut last: *const i32) {
    let mut comp: () = unimplemented!();
    let mut p: (*const i32, *const i32) = unimplemented!();
    if ((first) != (last)) {
        let mut j: *const i32 = first;
        'loop_: while ((j) != (last)) {
            assert!(!(unimplemented!()));
            assert!(!(unimplemented!()));
            j.prefix_inc();
        }
    } else {
        assert!(((p.0) == (last)));
        assert!(((p.1) == (last)));
    }
}
pub unsafe fn test_22(mut N: i32) {
    let mut a: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*a.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        let _first: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_2({ (a).cast_const() });
        let _last: forward_iterator_const_int_ptr_ =
            forward_iterator_const_int_ptr_::new_2({ (a.offset((N) as isize)).cast_const() });
        test_18(_first, _last)
    });
    {
        let __p = a;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test_23(mut N: i32) {
    let mut a: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*a.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        let _first: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_2({ (a).cast_const() });
        let _last: bidirectional_iterator_const_int_ptr_ =
            bidirectional_iterator_const_int_ptr_::new_2({ (a.offset((N) as isize)).cast_const() });
        test_19(_first, _last)
    });
    {
        let __p = a;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test_24(mut N: i32) {
    let mut a: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*a.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        let _first: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_2({ (a).cast_const() });
        let _last: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_2({ (a.offset((N) as isize)).cast_const() });
        test_20(_first, _last)
    });
    {
        let __p = a;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test_25(mut N: i32) {
    let mut a: *mut i32 =
        Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
    let mut i: i32 = 0;
    'loop_: while ((i) < (N)) {
        (*a.offset((i) as isize)) = i;
        i.prefix_inc();
    }
    unimplemented!();
    (unsafe {
        let _first: *const i32 = a;
        let _last: *const i32 = a.offset((N) as isize);
        test_21(_first, _last)
    });
    {
        let __p = a;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test_26() {
    (unsafe { test_22(0) });
    (unsafe { test_22(1) });
    (unsafe { test_22(2) });
    (unsafe { test_22(3) });
    (unsafe { test_22(10) });
    (unsafe { test_22(1000) });
    {
        let N: i32 = 100;
        let mut a: *mut i32 =
            Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            (*a.offset((i) as isize)) = 5;
            i.prefix_inc();
        }
        unimplemented!();
        let mut comp: () = unimplemented!();
        let mut p: (
            forward_iterator_const_int_ptr_,
            forward_iterator_const_int_ptr_,
        ) = unimplemented!();
        assert!(((unsafe { base_2(&p.0,) }) == ((a).cast_const())));
        assert!(
            ((unsafe { base_2(&p.1,) })
                == ((a.offset((N) as isize).offset(-((1) as isize))).cast_const()))
        );
        {
            let __p = a;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<i32>(),
                )))
            }
        };
    }
}
pub unsafe fn test_27() {
    (unsafe { test_23(0) });
    (unsafe { test_23(1) });
    (unsafe { test_23(2) });
    (unsafe { test_23(3) });
    (unsafe { test_23(10) });
    (unsafe { test_23(1000) });
    {
        let N: i32 = 100;
        let mut a: *mut i32 =
            Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            (*a.offset((i) as isize)) = 5;
            i.prefix_inc();
        }
        unimplemented!();
        let mut comp: () = unimplemented!();
        let mut p: (
            bidirectional_iterator_const_int_ptr_,
            bidirectional_iterator_const_int_ptr_,
        ) = unimplemented!();
        assert!(((unsafe { base_5(&p.0,) }) == ((a).cast_const())));
        assert!(
            ((unsafe { base_5(&p.1,) })
                == ((a.offset((N) as isize).offset(-((1) as isize))).cast_const()))
        );
        {
            let __p = a;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<i32>(),
                )))
            }
        };
    }
}
pub unsafe fn test_28() {
    (unsafe { test_24(0) });
    (unsafe { test_24(1) });
    (unsafe { test_24(2) });
    (unsafe { test_24(3) });
    (unsafe { test_24(10) });
    (unsafe { test_24(1000) });
    {
        let N: i32 = 100;
        let mut a: *mut i32 =
            Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            (*a.offset((i) as isize)) = 5;
            i.prefix_inc();
        }
        unimplemented!();
        let mut comp: () = unimplemented!();
        let mut p: (
            random_access_iterator_const_int_ptr_,
            random_access_iterator_const_int_ptr_,
        ) = unimplemented!();
        assert!(((unsafe { base_8(&p.0,) }) == ((a).cast_const())));
        assert!(
            ((unsafe { base_8(&p.1,) })
                == ((a.offset((N) as isize).offset(-((1) as isize))).cast_const()))
        );
        {
            let __p = a;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<i32>(),
                )))
            }
        };
    }
}
pub unsafe fn test_29() {
    (unsafe { test_25(0) });
    (unsafe { test_25(1) });
    (unsafe { test_25(2) });
    (unsafe { test_25(3) });
    (unsafe { test_25(10) });
    (unsafe { test_25(1000) });
    {
        let N: i32 = 100;
        let mut a: *mut i32 =
            Box::leak((0..(N as usize)).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            (*a.offset((i) as isize)) = 5;
            i.prefix_inc();
        }
        unimplemented!();
        let mut comp: () = unimplemented!();
        let mut p: (*const i32, *const i32) = unimplemented!();
        assert!(((unsafe { base_9(p.0,) }) == ((a).cast_const())));
        assert!(
            ((unsafe { base_9(p.1,) })
                == ((a.offset((N) as isize).offset(-((1) as isize))).cast_const()))
        );
        {
            let __p = a;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<i32>(),
                )))
            }
        };
    }
}
pub static mut il_30: std::cell::LazyCell<[i32; 8]> =
    std::cell::LazyCell::new(|| unsafe { [2, 4, 6, 8, 7, 5, 3, 1] });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct less {}
impl less {
    pub unsafe fn operator_call(&self, x: *const i32, y: *const i32) -> bool {
        return ((*x) < (*y));
    }
}
impl Callable2<*const i32, *const i32, bool> for less {
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe { less::operator_call(self, a1, a2) }
    }
}
pub unsafe fn constexpr_test_31() {
    let p: (*const i32, *const i32) = unimplemented!();
    const _: () = assert!(true, "*(p.first)  == 1");;
    const _: () = assert!(true, "*(p.second) == 8");;
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
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { constexpr_test_31() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const randomness_17);
    std::cell::LazyCell::force(&*&raw const il_30);
}
