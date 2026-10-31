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
    mut x: random_access_iterator_int_ptr_,
    mut n: isize,
) -> random_access_iterator_int_ptr_ {
    (unsafe { random_access_iterator_int_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_int_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_1(
    mut x: random_access_iterator_int_ptr_,
    mut y: random_access_iterator_int_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_lt_2(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> bool {
    return (((*x).it_) < ((*y).it_));
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
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_int_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_int_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_int_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
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
impl std::cmp::Ord for random_access_iterator_int_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_2(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_2(
                other as *const random_access_iterator_int_ptr_,
                self as *const random_access_iterator_int_ptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_int_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_int_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_2(
                self as *const random_access_iterator_int_ptr_,
                other as *const random_access_iterator_int_ptr_,
            )) && !(operator_lt_2(
                other as *const random_access_iterator_int_ptr_,
                self as *const random_access_iterator_int_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for random_access_iterator_int_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_int_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_8();
    }
}
pub unsafe fn operator_add_3(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut n: isize,
) -> random_access_iterator_MoveOnly_ptr_ {
    (unsafe { random_access_iterator_MoveOnly_ptr_::operator_add_assign(&mut x, n) });
    return random_access_iterator_MoveOnly_ptr_::move_from({ &mut x });
}
pub unsafe fn operator_sub_4(
    mut x: random_access_iterator_MoveOnly_ptr_,
    mut y: random_access_iterator_MoveOnly_ptr_,
) -> isize {
    return (((x.it_ as usize - y.it_ as usize) / ::std::mem::size_of::<MoveOnly>()) as i64
        as isize);
}
pub unsafe fn operator_lt_5(
    x: *const random_access_iterator_MoveOnly_ptr_,
    y: *const random_access_iterator_MoveOnly_ptr_,
) -> bool {
    return (((*x).it_) < ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_MoveOnly_ptr_ {
    it_: *mut MoveOnly,
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_MoveOnly_ptr_ {
    pub unsafe fn new_1(mut it: *mut MoveOnly) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *mut MoveOnly {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_inc_7(&mut self) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_.prefix_inc();
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_dec_8(&mut self) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_.prefix_dec();
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_ = (self.it_).wrapping_add(n as usize);
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
    pub unsafe fn move_from(_a0: *mut random_access_iterator_MoveOnly_ptr_) -> Self {
        let mut this = Self {
            it_: (*_a0).it_,
            tracker_: support_double_move_tracker::move_from({ &mut (*_a0).tracker_ }),
        };
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut random_access_iterator_MoveOnly_ptr_,
    ) -> *mut random_access_iterator_MoveOnly_ptr_ {
        self.it_ = (*_a0).it_;
        (unsafe {
            let _other: *mut support_double_move_tracker = &mut (*_a0).tracker_;
            support_double_move_tracker::move_assign(&mut self.tracker_, _other)
        });
        return &mut (*(self as *mut random_access_iterator_MoveOnly_ptr_));
    }
}
impl std::cmp::Ord for random_access_iterator_MoveOnly_ptr_ {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_5(
                self as *const random_access_iterator_MoveOnly_ptr_,
                other as *const random_access_iterator_MoveOnly_ptr_,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_5(
                other as *const random_access_iterator_MoveOnly_ptr_,
                self as *const random_access_iterator_MoveOnly_ptr_,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for random_access_iterator_MoveOnly_ptr_ {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for random_access_iterator_MoveOnly_ptr_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_5(
                self as *const random_access_iterator_MoveOnly_ptr_,
                other as *const random_access_iterator_MoveOnly_ptr_,
            )) && !(operator_lt_5(
                other as *const random_access_iterator_MoveOnly_ptr_,
                self as *const random_access_iterator_MoveOnly_ptr_,
            ))
        }
    }
}
impl std::cmp::Eq for random_access_iterator_MoveOnly_ptr_ {}
impl UnsafeCppAdvance for random_access_iterator_MoveOnly_ptr_ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_7();
    }
    unsafe fn cpp_dec(&mut self) {
        self.operator_dec_8();
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
pub unsafe fn operator_eq_13(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_14(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_15(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_16(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_17(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_18(x: *const MoveOnly, y: *const MoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_19(_a0: *const MoveOnly, _a1: *const MoveOnly) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const MoveOnly) -> MoveOnly {
        return MoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for MoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { operator_cmp_19(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::PartialOrd for MoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for MoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_13(self as *const MoveOnly, other as *const MoveOnly) }
    }
}
impl std::cmp::Eq for MoveOnly {}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn operator_eq_20(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_21(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) != ((*y).data_));
}
pub unsafe fn operator_lt_22(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) < ((*y).data_));
}
pub unsafe fn operator_le_23(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) <= ((*y).data_));
}
pub unsafe fn operator_gt_24(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) > ((*y).data_));
}
pub unsafe fn operator_ge_25(x: *const TrivialMoveOnly, y: *const TrivialMoveOnly) -> bool {
    return (((*x).data_) >= ((*y).data_));
}
pub unsafe fn operator_cmp_26(
    _a0: *const TrivialMoveOnly,
    _a1: *const TrivialMoveOnly,
) -> std::cmp::Ordering {
    {
        let mut cmp: std::cmp::Ordering = std::cmp::Ord::cmp(&((*_a0).data_), &((*_a1).data_));
        if !(cmp == std::cmp::Ordering::Equal) {
            return cmp;
        }
    }
    return std::cmp::Ordering::Equal;
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct TrivialMoveOnly {
    data_: i32,
}
impl TrivialMoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(_a0: *mut TrivialMoveOnly) -> Self {
        let mut this = Self {
            data_: (*_a0).data_,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut TrivialMoveOnly) -> *mut TrivialMoveOnly {
        self.data_ = (*_a0).data_;
        return &mut (*(self as *mut TrivialMoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
        return self.data_;
    }
    pub unsafe fn operator_add(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) + ((*x).data_))) });
    }
    pub unsafe fn operator_mul(&self, x: *const TrivialMoveOnly) -> TrivialMoveOnly {
        return TrivialMoveOnly::new({ Some(((self.data_) * ((*x).data_))) });
    }
}
impl std::cmp::Ord for TrivialMoveOnly {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_26(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::PartialOrd for TrivialMoveOnly {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for TrivialMoveOnly {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_20(
                self as *const TrivialMoveOnly,
                other as *const TrivialMoveOnly,
            )
        }
    }
}
impl std::cmp::Eq for TrivialMoveOnly {}
impl Default for TrivialMoveOnly {
    fn default() -> Self {
        unsafe { TrivialMoveOnly::new(None) }
    }
}
pub unsafe fn test_27() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut n: i32 = 0;
    'loop_: while ((n) < (15)) {
        let mut m: i32 = 0;
        'loop_: while ((m) <= (n)) {
            unimplemented!();
            assert!(unimplemented!());
            assert!(unimplemented!());
            let mut i: i32 = m;
            'loop_: while ((i) < (n)) {
                assert!(((m) == (0)) || (!((work[(i) as usize]) < (work[((m) - (1)) as usize]))));
                i.prefix_inc();
            }
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((15) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() = (*__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            m.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [i32; 5] = [3, 4, 2, 5, 1];
        unimplemented!();
        assert!(((input[(0) as usize]) == (1)));
        assert!(((input[(1) as usize]) == (2)));
        assert!(((input[(2) as usize]) == (3)));
        assert!((((input[(3) as usize]) + (input[(4) as usize])) == ((4) + (5))));
    }
    return true;
}
pub unsafe fn test_28() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut n: i32 = 0;
    'loop_: while ((n) < (15)) {
        let mut m: i32 = 0;
        'loop_: while ((m) <= (n)) {
            unimplemented!();
            assert!(unimplemented!());
            assert!(unimplemented!());
            let mut i: i32 = m;
            'loop_: while ((i) < (n)) {
                assert!(((m) == (0)) || (!((work[(i) as usize]) < (work[((m) - (1)) as usize]))));
                i.prefix_inc();
            }
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((15) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() = (*__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            m.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [i32; 5] = [3, 4, 2, 5, 1];
        unimplemented!();
        assert!(((input[(0) as usize]) == (1)));
        assert!(((input[(1) as usize]) == (2)));
        assert!(((input[(2) as usize]) == (3)));
        assert!((((input[(3) as usize]) + (input[(4) as usize])) == ((4) + (5))));
    }
    return true;
}
pub unsafe fn test_29() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [MoveOnly; 15] = [
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ];
    let mut n: i32 = 0;
    'loop_: while ((n) < (15)) {
        let mut m: i32 = 0;
        'loop_: while ((m) <= (n)) {
            unimplemented!();
            assert!(unimplemented!());
            assert!(unimplemented!());
            let mut i: i32 = m;
            'loop_: while ((i) < (n)) {
                assert!(
                    ((m) == (0))
                        || (!(unsafe {
                            let _x: *const MoveOnly = &work[(i) as usize];
                            let _y: *const MoveOnly = &work[((m) - (1)) as usize];
                            operator_lt_15(_x, _y)
                        }))
                );
                i.prefix_inc();
            }
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((15) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() =
                        <MoveOnly as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            m.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [MoveOnly; 5] = [
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(1) }),
        ];
        unimplemented!();
        assert!(
            (unsafe {
                let _x: *const MoveOnly = &input[(0) as usize];
                let mut _y: MoveOnly = MoveOnly::new({ Some(1) });
                operator_eq_13(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const MoveOnly = &input[(1) as usize];
                let mut _y: MoveOnly = MoveOnly::new({ Some(2) });
                operator_eq_13(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const MoveOnly = &input[(2) as usize];
                let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                operator_eq_13(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let mut _x: MoveOnly = (unsafe {
                    let _x: *const MoveOnly = &input[(4) as usize];
                    MoveOnly::operator_add(&input[(3) as usize], _x)
                });
                let mut _y: MoveOnly = MoveOnly::new({ Some(((4) + (5))) });
                operator_eq_13(&mut _x, &mut _y)
            })
        );
    }
    return true;
}
pub unsafe fn test_30() -> bool {
    let mut orig: [i32; 15] = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9];
    let mut work: [MoveOnly; 15] = [
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(4) }),
        MoveOnly::new({ Some(1) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(2) }),
        MoveOnly::new({ Some(6) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(3) }),
        MoveOnly::new({ Some(5) }),
        MoveOnly::new({ Some(8) }),
        MoveOnly::new({ Some(9) }),
        MoveOnly::new({ Some(7) }),
        MoveOnly::new({ Some(9) }),
    ];
    let mut n: i32 = 0;
    'loop_: while ((n) < (15)) {
        let mut m: i32 = 0;
        'loop_: while ((m) <= (n)) {
            unimplemented!();
            assert!(unimplemented!());
            assert!(unimplemented!());
            let mut i: i32 = m;
            'loop_: while ((i) < (n)) {
                assert!(
                    ((m) == (0))
                        || (!(unsafe {
                            let _x: *const MoveOnly = &work[(i) as usize];
                            let _y: *const MoveOnly = &work[((m) - (1)) as usize];
                            operator_lt_15(_x, _y)
                        }))
                );
                i.prefix_inc();
            }
            {
                let mut __first = orig.as_mut_ptr();
                let __last = orig.as_mut_ptr().offset((15) as isize);
                let mut __out = work.as_mut_ptr();
                while __first != __last {
                    *__out.iter_deref() =
                        <MoveOnly as CppFromUnsafe<i32>>::cpp_from(__first.iter_deref());
                    __first.iter_inc();
                    __out.iter_inc();
                }
                __out
            };
            m.prefix_inc();
        }
        n.prefix_inc();
    }
    {
        let mut input: [MoveOnly; 5] = [
            MoveOnly::new({ Some(3) }),
            MoveOnly::new({ Some(4) }),
            MoveOnly::new({ Some(2) }),
            MoveOnly::new({ Some(5) }),
            MoveOnly::new({ Some(1) }),
        ];
        unimplemented!();
        assert!(
            (unsafe {
                let _x: *const MoveOnly = &input[(0) as usize];
                let mut _y: MoveOnly = MoveOnly::new({ Some(1) });
                operator_eq_13(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const MoveOnly = &input[(1) as usize];
                let mut _y: MoveOnly = MoveOnly::new({ Some(2) });
                operator_eq_13(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let _x: *const MoveOnly = &input[(2) as usize];
                let mut _y: MoveOnly = MoveOnly::new({ Some(3) });
                operator_eq_13(_x, &mut _y)
            })
        );
        assert!(
            (unsafe {
                let mut _x: MoveOnly = (unsafe {
                    let _x: *const MoveOnly = &input[(4) as usize];
                    MoveOnly::operator_add(&input[(3) as usize], _x)
                });
                let mut _y: MoveOnly = MoveOnly::new({ Some(((4) + (5))) });
                operator_eq_13(&mut _x, &mut _y)
            })
        );
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
    let mut i: i32 = 42;
    unimplemented!();
    assert!(((i) == (42)));
    (unsafe { test_27() });
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
    const _: () = assert!(true, "test<int, random_access_iterator<int*>>()");;
    const _: () = assert!(true, "test<int, int*>()");;
    const _: () = assert!(true, "test<MoveOnly, random_access_iterator<MoveOnly*>>()");;
    const _: () = assert!(true, "test<MoveOnly, MoveOnly*>()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
