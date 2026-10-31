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
pub unsafe fn operator_add_0(
    mut x: random_access_iterator_const_int_ptr_,
    mut n: isize,
) -> random_access_iterator_const_int_ptr_ {
    (unsafe { random_access_iterator_const_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_1(
    mut x: random_access_iterator_const_int_ptr_,
    mut y: random_access_iterator_const_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_2(
    x: *const random_access_iterator_const_int_ptr_,
    y: *const random_access_iterator_const_int_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
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
    pub unsafe fn operator_index(&self, mut n: isize) -> *const i32 {
        return &(*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_const_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
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
            operator_eq_2(
                self as *const random_access_iterator_const_int_ptr_,
                other as *const random_access_iterator_const_int_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_int_ptr_ {}
pub unsafe fn operator_add_3(
    mut x: random_access_iterator_const_char_ptr_,
    mut n: isize,
) -> random_access_iterator_const_char_ptr_ {
    (unsafe { random_access_iterator_const_char_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_const_char_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_4(
    mut x: random_access_iterator_const_char_ptr_,
    mut y: random_access_iterator_const_char_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<libc::c_char>()) as i64
        as isize);
}
pub unsafe fn operator_eq_5(
    x: *const random_access_iterator_const_char_ptr_,
    y: *const random_access_iterator_const_char_ptr_,
) -> bool {
    return (((*x).it_) == ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_const_char_ptr_ {
    it_: *const libc::c_char,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_const_char_ptr_ {
    pub unsafe fn new_1(mut it: *const libc::c_char) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_index(&self, mut n: isize) -> *const libc::c_char {
        return &(*self.it_.offset((n) as isize));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_const_char_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_const_char_ptr_,
    ) -> *mut random_access_iterator_const_char_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_const_char_ptr_));
    }
}
impl std::cmp::PartialEq for random_access_iterator_const_char_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_5(
                self as *const random_access_iterator_const_char_ptr_,
                other as *const random_access_iterator_const_char_ptr_,
            )
        }
    }
}
impl std::cmp::Eq for random_access_iterator_const_char_ptr_ {}
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
pub unsafe fn operator_eq_6(
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
            operator_eq_6(
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
pub unsafe fn operator_add_7(mut i: adl_Iterator, mut n: isize) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_8(mut n: isize, mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_7(_i, n)
    });
}
pub unsafe fn iter_swap_9(mut a: adl_Iterator, mut b: adl_Iterator) {
    cpp_swap_unsafe!(a.ptr_, b.ptr_);
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_10(mut iter: adl_Iterator) -> *mut i32 {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_11(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> bool {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_12(
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
        unsafe { operator_cmp_12(self as *const adl_Iterator, other as *const adl_Iterator) }
    }
}
impl std::cmp::PartialOrd for adl_Iterator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for adl_Iterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_11(self as *const adl_Iterator, other as *const adl_Iterator) }
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
pub struct MyHash_int_ {}
impl MyHash_int_ {
    pub unsafe fn operator_call(&self, mut t: i32) -> usize {
        return (t as usize);
    }
}
impl Callable1<i32, usize> for MyHash_int_ {
    fn call(&self, a1: i32) -> usize {
        unsafe { MyHash_int_::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct MyHash_char_ {}
impl MyHash_char_ {
    pub unsafe fn operator_call(&self, mut t: libc::c_char) -> usize {
        return (t as usize);
    }
}
impl Callable1<libc::c_char, usize> for MyHash_char_ {
    fn call(&self, a1: libc::c_char) -> usize {
        unsafe { MyHash_char_::operator_call(self, a1) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct count_equal {}
impl count_equal {
    pub unsafe fn operator_call_1_int(&self, x: *const i32, y: *const i32) -> bool {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_inc();
        return ((*x) == (*y));
    }
    pub unsafe fn operator_call_1_char(
        &self,
        x: *const libc::c_char,
        y: *const libc::c_char,
    ) -> bool {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)).prefix_inc();
        return (((*x) as i32) == ((*y) as i32));
    }
}
impl Callable2<*const i32, *const i32, bool> for count_equal {
    fn call(&self, a1: *const i32, a2: *const i32) -> bool {
        unsafe { count_equal::operator_call_1_int(self, a1, a2) }
    }
}
impl Callable2<*const libc::c_char, *const libc::c_char, bool> for count_equal {
    fn call(&self, a1: *const libc::c_char, a2: *const libc::c_char) -> bool {
        unsafe { count_equal::operator_call_1_char(self, a1, a2) }
    }
}
pub static mut count_13: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| unsafe { 0_u32 });
pub unsafe fn do_search_14(
    mut b1: random_access_iterator_const_int_ptr_,
    mut e1: random_access_iterator_const_int_ptr_,
    mut b2: random_access_iterator_const_int_ptr_,
    mut e2: random_access_iterator_const_int_ptr_,
    mut result: random_access_iterator_const_int_ptr_,
    mut max_count: u32,
) {
    let mut s: () = unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) = 0_u32;
    assert!(
        (unsafe {
            let _x: *const random_access_iterator_const_int_ptr_ = &result;
            let mut _y: random_access_iterator_const_int_ptr_ = unimplemented!();
            operator_eq_2(_x, &mut _y)
        })
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) <= (max_count)));
}
pub unsafe fn do_search_15(
    mut b1: random_access_iterator_const_char_ptr_,
    mut e1: random_access_iterator_const_char_ptr_,
    mut b2: random_access_iterator_const_char_ptr_,
    mut e2: random_access_iterator_const_char_ptr_,
    mut result: random_access_iterator_const_char_ptr_,
    mut max_count: u32,
) {
    let mut s: () = unimplemented!();
    (*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) = 0_u32;
    assert!(
        (unsafe {
            let _x: *const random_access_iterator_const_char_ptr_ = &result;
            let mut _y: random_access_iterator_const_char_ptr_ = unimplemented!();
            operator_eq_5(_x, &mut _y)
        })
    );
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut count_13)) <= (max_count)));
}
pub unsafe fn test_16() {
    let mut ia: [i32; 6] = [0, 1, 2, 3, 4, 5];
    let sa: u32 = (((::std::mem::size_of::<[i32; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_14(_b1, _e1, _b2, _e2, _result, 0_u32)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _max_count: u32 = sa;
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_14(_b1, _e1, _b2, _e2, _result, 0_u32)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_14(_b1, _e1, _b2, _e2, _result, 0_u32)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _max_count: u32 = sa;
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3) as isize)))
                .cast_const()
            });
        let _max_count: u32 = (3_u32).wrapping_mul(sa);
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _max_count: u32 = (sa).wrapping_mul(sa);
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _max_count: u32 = (((sa).wrapping_sub(1_u32)) as u32).wrapping_mul(sa);
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ib: [i32; 12] = [0, 1, 2, 0, 1, 2, 3, 0, 1, 2, 3, 4];
    let sb: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ic: [i32; 1] = [1];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ic.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ic.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = sb;
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut id: [i32; 2] = [1, 2];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (id.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (id.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = (sb).wrapping_mul(2_u32);
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ie: [i32; 3] = [1, 2, 3];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ie.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ie.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((4) as isize)).cast_const()
            });
        let _max_count: u32 = (sb).wrapping_mul(3_u32);
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ig: [i32; 4] = [1, 2, 3, 4];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ig.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ig.as_mut_ptr().offset((4) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ib.as_mut_ptr().offset((8) as isize)).cast_const()
            });
        let _max_count: u32 = (sb).wrapping_mul(4_u32);
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ih: [i32; 12] = [0, 1, 1, 1, 1, 2, 3, 0, 1, 2, 3, 4];
    let sh: u32 = (((::std::mem::size_of::<[i32; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<i32>() as usize))) as u32);
    let mut ii: [i32; 3] = [1, 1, 2];
    (unsafe {
        let _b1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ih.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ih.as_mut_ptr().offset((sh) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({ (ii.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ii.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_int_ptr_ =
            random_access_iterator_const_int_ptr_::new_1({
                (ih.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _max_count: u32 = (sh).wrapping_mul(3_u32);
        do_search_14(_b1, _e1, _b2, _e2, _result, _max_count)
    });
}
pub unsafe fn test2_17() {
    let mut ia: [libc::c_char; 6] = [
        (0 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
        (5 as libc::c_char),
    ];
    let sa: u32 = (((::std::mem::size_of::<[libc::c_char; 6]>() as usize)
        .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize))) as u32);
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_15(_b1, _e1, _b2, _e2, _result, 0_u32)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _max_count: u32 = sa;
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_15(_b1, _e1, _b2, _e2, _result, 0_u32)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        do_search_15(_b1, _e1, _b2, _e2, _result, 0_u32)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _max_count: u32 = sa;
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3) as isize)))
                .cast_const()
            });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((3) as isize)))
                .cast_const()
            });
        let _max_count: u32 = (3_u32).wrapping_mul(sa);
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _max_count: u32 = (sa).wrapping_mul(sa);
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr()
                    .offset((sa) as isize)
                    .offset(-((1) as isize)))
                .cast_const()
            });
        let _max_count: u32 = (((sa).wrapping_sub(1_u32)) as u32).wrapping_mul(sa);
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ia.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((sa) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ia.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = sa;
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ib: [libc::c_char; 12] = [
        (0 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
        (0 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (0 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
    ];
    let sb: u32 = (((::std::mem::size_of::<[libc::c_char; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize))) as u32);
    let mut ic: [libc::c_char; 1] = [(1 as libc::c_char)];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ic.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ic.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = sb;
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut id: [libc::c_char; 2] = [(1 as libc::c_char), (2 as libc::c_char)];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (id.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (id.as_mut_ptr().offset((2) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((1) as isize)).cast_const()
            });
        let _max_count: u32 = (sb).wrapping_mul(2_u32);
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ie: [libc::c_char; 3] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
    ];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ie.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ie.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((4) as isize)).cast_const()
            });
        let _max_count: u32 = (sb).wrapping_mul(3_u32);
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ig: [libc::c_char; 4] = [
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
    ];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ib.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((sb) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ig.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ig.as_mut_ptr().offset((4) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ib.as_mut_ptr().offset((8) as isize)).cast_const()
            });
        let _max_count: u32 = (sb).wrapping_mul(4_u32);
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
    let mut ih: [libc::c_char; 12] = [
        (0 as libc::c_char),
        (1 as libc::c_char),
        (1 as libc::c_char),
        (1 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (0 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
        (3 as libc::c_char),
        (4 as libc::c_char),
    ];
    let sh: u32 = (((::std::mem::size_of::<[libc::c_char; 12]>() as usize)
        .wrapping_div((::std::mem::size_of::<libc::c_char>() as usize))) as u32);
    let mut ii: [libc::c_char; 3] = [
        (1 as libc::c_char),
        (1 as libc::c_char),
        (2 as libc::c_char),
    ];
    (unsafe {
        let _b1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ih.as_mut_ptr()).cast_const() });
        let _e1: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ih.as_mut_ptr().offset((sh) as isize)).cast_const()
            });
        let _b2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({ (ii.as_mut_ptr()).cast_const() });
        let _e2: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ii.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _result: random_access_iterator_const_char_ptr_ =
            random_access_iterator_const_char_ptr_::new_1({
                (ih.as_mut_ptr().offset((3) as isize)).cast_const()
            });
        let _max_count: u32 = (sh).wrapping_mul(3_u32);
        do_search_15(_b1, _e1, _b2, _e2, _result, _max_count)
    });
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
    (unsafe { test_16() });
    (unsafe { test2_17() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_13);
}
