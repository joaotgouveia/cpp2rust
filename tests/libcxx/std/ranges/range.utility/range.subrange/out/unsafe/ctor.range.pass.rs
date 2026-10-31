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
pub unsafe fn base_0(i: *const forward_iterator_int_ptr_) -> *mut i32 {
    return (*i).it_;
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
pub unsafe fn operator_eq_1(
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
            operator_eq_1(
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
pub unsafe fn operator_add_2(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_3(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_2(_i, n)
    });
}
pub unsafe fn iter_swap_4(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_5(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_6(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_7(
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
        unsafe { operator_cmp_7(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_6(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub static mut globalBuff_8: std::cell::LazyCell<[i32; 8]> =
    std::cell::LazyCell::new(|| unsafe { [0_i32; 8] });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
pub unsafe fn operator_eq_9(lhs: *const MoveOnlyForwardIter, mut rhs: *mut i32) -> bool {
    return (((*lhs).base) == (rhs));
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(*mut i32 , new_1, value)]
#[cpp_into(*mut i32 , to_pmuti32)]
pub struct MoveOnlyForwardIter {
    pub base: *mut i32,
}
impl MoveOnlyForwardIter {
    pub unsafe fn move_from(_a0: *mut MoveOnlyForwardIter) -> Self {
        let mut this = Self { base: (*_a0).base };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut MoveOnlyForwardIter,
    ) -> *mut MoveOnlyForwardIter {
        self.base = (*_a0).base;
        return &mut (*(self as *mut MoveOnlyForwardIter));
    }
    pub unsafe fn new_1(mut ptr: *mut i32) -> Self {
        let mut this = Self { base: ptr };
        this
    }
    pub unsafe fn to_pmuti32(&self) -> *mut i32 {
        return self.base;
    }
}
impl Default for MoveOnlyForwardIter {
    fn default() -> Self {
        unsafe {
            MoveOnlyForwardIter {
                base: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_10(
    lhs: *const SizedSentinelForwardIter,
    rhs: *const SizedSentinelForwardIter,
) -> bool {
    return (((*lhs).base_) == ((*rhs).base_));
}
pub unsafe fn operator_sub_11(
    a: *const SizedSentinelForwardIter,
    b: *const SizedSentinelForwardIter,
) -> isize {
    if !((*a).minusWasCalled_).is_null() {
        (*(*a).minusWasCalled_) = true;
    }
    if !((*b).minusWasCalled_).is_null() {
        (*(*b).minusWasCalled_) = true;
    }
    return ((((*a).base_ as usize - (*b).base_ as usize) / ::std::mem::size_of::<i32>()) as i64
        as isize);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct SizedSentinelForwardIter {
    base_: *mut i32,
    minusWasCalled_: *mut bool,
}
impl SizedSentinelForwardIter {
    pub unsafe fn new_1(mut ptr: *mut i32, mut minusWasCalled: *mut bool) -> Self {
        let mut this = Self {
            base_: ptr,
            minusWasCalled_: minusWasCalled,
        };
        this
    }
}
impl std::cmp::PartialEq for SizedSentinelForwardIter {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_10(
                self as *const SizedSentinelForwardIter,
                other as *const SizedSentinelForwardIter,
            )
        }
    }
}
impl std::cmp::Eq for SizedSentinelForwardIter {}
impl Default for SizedSentinelForwardIter {
    fn default() -> Self {
        unsafe {
            SizedSentinelForwardIter {
                base_: std::ptr::null_mut(),
                minusWasCalled_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(
    true,
    "std::sized_sentinel_for<SizedSentinelForwardIter, SizedSentinelForwardIter>"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe)]
#[cpp_into(*mut i32 , to_pmuti32)]
pub struct ConvertibleForwardIter {
    pub base_: *mut i32,
}
impl ConvertibleForwardIter {
    pub unsafe fn new_1(mut ptr: *mut i32) -> Self {
        let mut this = Self { base_: ptr };
        this
    }
    pub unsafe fn to_pmuti32(&self) -> *mut i32 {
        return self.base_;
    }
}
impl Default for ConvertibleForwardIter {
    fn default() -> Self {
        unsafe {
            ConvertibleForwardIter {
                base_: std::ptr::null_mut(),
            }
        }
    }
}
const _: () = assert!(true, "std::is_convertible_v<ConvertibleForwardIter, int*>");
pub unsafe fn operator_eq_12(
    _a0: *const ConditionallyConvertibleBase_false_,
    _a1: *const ConditionallyConvertibleBase_false_,
) -> bool {
    return (((*_a0).base_) == ((*_a1).base_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConditionallyConvertibleBase_false_ {
    pub base_: *mut i32,
}
impl ConditionallyConvertibleBase_false_ {}
impl std::cmp::PartialEq for ConditionallyConvertibleBase_false_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_12(
                self as *const ConditionallyConvertibleBase_false_,
                other as *const ConditionallyConvertibleBase_false_,
            )
        }
    }
}
impl std::cmp::Eq for ConditionallyConvertibleBase_false_ {}
impl Default for ConditionallyConvertibleBase_false_ {
    fn default() -> Self {
        unsafe {
            ConditionallyConvertibleBase_false_ {
                base_: std::ptr::null_mut(),
            }
        }
    }
}
pub unsafe fn operator_eq_13(
    _a0: *const ConditionallyConvertibleBase_true_,
    _a1: *const ConditionallyConvertibleBase_true_,
) -> bool {
    return (((*_a0).base_) == ((*_a1).base_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ConditionallyConvertibleBase_true_ {
    pub base_: *mut i32,
}
impl ConditionallyConvertibleBase_true_ {}
impl std::cmp::PartialEq for ConditionallyConvertibleBase_true_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
                self as *const ConditionallyConvertibleBase_true_,
                other as *const ConditionallyConvertibleBase_true_,
            )
        }
    }
}
impl std::cmp::Eq for ConditionallyConvertibleBase_true_ {}
impl Default for ConditionallyConvertibleBase_true_ {
    fn default() -> Self {
        unsafe {
            ConditionallyConvertibleBase_true_ {
                base_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardBorrowedRange {}
impl ForwardBorrowedRange {
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8)).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8))
                .as_mut_ptr()
                .offset((8) as isize)
        });
    }
}
pub static mut enable_borrowed_range_14: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { true });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardRange {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ConvertibleForwardBorrowedRange {}
impl ConvertibleForwardBorrowedRange {
    pub unsafe fn begin(&self) -> ConvertibleForwardIter {
        return ConvertibleForwardIter::new_1({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8)).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> *mut i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8))
            .as_mut_ptr()
            .offset((8) as isize);
    }
}
pub static mut enable_borrowed_range_15: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { true });
pub unsafe fn operator_eq_16(
    mut s: ForwardBorrowedRangeDifferentSentinel_sentinel,
    mut i: forward_iterator_int_ptr_,
) -> bool {
    return ((s.value) == (unsafe { base_0(&i) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardBorrowedRangeDifferentSentinel_sentinel {
    pub value: *mut i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ForwardBorrowedRangeDifferentSentinel {}
impl ForwardBorrowedRangeDifferentSentinel {
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8)).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> ForwardBorrowedRangeDifferentSentinel_sentinel {
        return ForwardBorrowedRangeDifferentSentinel_sentinel {
            value: (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8))
                .as_mut_ptr()
                .offset((8) as isize),
        };
    }
}
pub static mut enable_borrowed_range_17: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { true });
pub unsafe fn operator_eq_18(
    mut s: DifferentSentinelWithSizeMember_sentinel,
    mut i: forward_iterator_int_ptr_,
) -> bool {
    return ((s.value) == (unsafe { base_0(&i) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DifferentSentinelWithSizeMember_sentinel {
    pub value: *mut i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct DifferentSentinelWithSizeMember {}
impl DifferentSentinelWithSizeMember {
    pub unsafe fn begin(&self) -> forward_iterator_int_ptr_ {
        return forward_iterator_int_ptr_::new_1({
            (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8)).as_mut_ptr()
        });
    }
    pub unsafe fn end(&self) -> DifferentSentinelWithSizeMember_sentinel {
        return DifferentSentinelWithSizeMember_sentinel {
            value: (*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8))
                .as_mut_ptr()
                .offset((8) as isize),
        };
    }
    pub unsafe fn size(&self) -> usize {
        return 8_usize;
    }
}
pub static mut enable_borrowed_range_19: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { true });
const _: () = assert!(
    true,
    "std::is_constructible_v<ForwardSubrange, ForwardBorrowedRange>"
);
const _: () = assert!(
    true,
    "!std::is_constructible_v<ForwardSubrange, ForwardRange>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<ConvertibleForwardSubrange, ConvertibleForwardBorrowedRange>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<DifferentSentinelSubrange, ForwardBorrowedRangeDifferentSentinel>"
);
const _: () = assert!(
    true,
    "std::is_constructible_v<DifferentSentinelWithSizeMemberSubrange, DifferentSentinelWithSizeMember>"
);
pub unsafe fn test_20() -> bool {
    let mut a: () = unimplemented!();
    assert!(
        ((unsafe {
            let mut _i: forward_iterator_int_ptr_ = unimplemented!();
            base_0(&mut _i)
        }) == ((*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8)).as_mut_ptr()))
    );
    assert!(
        ((unsafe {
            let mut _i: forward_iterator_int_ptr_ = unimplemented!();
            base_0(&mut _i)
        }) == ((*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8))
            .as_mut_ptr()
            .offset((8) as isize)))
    );
    let mut b: () = unimplemented!();
    assert!(
        ((unsafe { ConvertibleForwardIter::to_pmuti32(&unimplemented!(),) })
            == ((*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8)).as_mut_ptr()))
    );
    assert!(
        ((unimplemented!())
            == ((*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8))
                .as_mut_ptr()
                .offset((8) as isize)))
    );
    let mut c: () = unimplemented!();
    assert!(
        ((unsafe {
            let mut _i: forward_iterator_int_ptr_ = unimplemented!();
            base_0(&mut _i)
        }) == ((*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8)).as_mut_ptr()))
    );
    assert!(
        ((unimplemented!().value)
            == ((*std::cell::LazyCell::force_mut(&mut *&raw mut globalBuff_8))
                .as_mut_ptr()
                .offset((8) as isize)))
    );
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
    (unsafe { test_20() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const globalBuff_8);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_14);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_15);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_17);
    std::cell::LazyCell::force(&*&raw const enable_borrowed_range_19);
}
