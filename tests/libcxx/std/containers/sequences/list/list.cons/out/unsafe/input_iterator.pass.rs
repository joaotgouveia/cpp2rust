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
    moved_from_: (),
}
impl support_double_move_tracker {
    pub unsafe fn new() -> Self {
        let mut this = Self { moved_from_: false };
        this
    }
    pub unsafe fn move_from(other: *mut support_double_move_tracker) -> Self {
        let mut this = Self { moved_from_: false };
        unimplemented!();
        (*other).moved_from_ = true;
        this
    }
    pub unsafe fn move_assign(
        &mut self,
        other: *mut support_double_move_tracker,
    ) -> *mut support_double_move_tracker {
        unimplemented!();
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
    it_: *mut (),
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
pub unsafe fn operator_ne_0(
    x: *const cpp17_input_iterator_int_ptr__int_ptr_,
    y: *const cpp17_input_iterator_int_ptr__int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_int_ptr__int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_int_ptr__int_ptr_ {
    pub unsafe fn new(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
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
pub unsafe fn operator_ne_1(
    x: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
    y: *const cpp17_input_iterator_const_int_ptr__const_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    it_: *const (),
    tracker_: support_double_move_tracker,
}
impl cpp17_input_iterator_const_int_ptr__const_int_ptr_ {
    pub unsafe fn new(mut it: *const ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
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
const _: () = assert!(true, "std::input_iterator<cpp17_input_iterator<int*>>");
pub unsafe fn operator_ne_2(
    x: *const forward_iterator_int_ptr_,
    y: *const forward_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct forward_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl forward_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
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
pub unsafe fn operator_ne_3(
    x: *const random_access_iterator_int_ptr_,
    y: *const random_access_iterator_int_ptr_,
) -> () {
    return (((*x).it_) != ((*y).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl random_access_iterator_int_ptr_ {
    pub unsafe fn new_1(mut it: *mut ()) -> Self {
        let mut this = Self {
            it_: it,
            tracker_: support_double_move_tracker::new(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> () {
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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_random_access_iterator_int_ptr_ {
    it_: *mut (),
    tracker_: support_double_move_tracker,
}
impl cpp20_random_access_iterator_int_ptr_ {
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
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
    it_: *mut (),
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
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.it_);
    }
}
const _: () = assert!(true, "std::input_iterator<cpp20_input_iterator<int*>>");
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct cpp20_output_iterator_int_ptr_ {
    it_: *mut (),
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
    pub unsafe fn operator_deref(&self) -> *mut () {
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
    pub increments: (),
    pub decrements: (),
    pub zero_moves: (),
    pub equal_cmps: (),
}
impl Default for IteratorOpCounts {
    fn default() -> Self {
        unsafe {
            IteratorOpCounts {
                increments: 0_(),
                decrements: 0_(),
                zero_moves: 0_(),
                equal_cmps: 0_(),
            }
        }
    }
}
pub unsafe fn operator_eq_4(
    _a0: *const forward_sized_iterator_int_ptr_,
    _a1: *const forward_sized_iterator_int_ptr_,
) -> () {
    return (((*_a0).it_) == ((*_a1).it_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct forward_sized_iterator_int_ptr_ {
    pub it_: *mut (),
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
pub unsafe fn operator_add_5(mut i: adl_Iterator, mut n: ()) -> adl_Iterator {
    return adl_Iterator::new_1({ i.ptr_.offset((n) as isize) }, { i.iter_moves_ }, {
        i.iter_swaps_
    });
}
pub unsafe fn operator_add_6(mut n: (), mut i: adl_Iterator) -> adl_Iterator {
    return (unsafe {
        let _i: adl_Iterator = i;
        operator_add_5(_i, n)
    });
}
pub unsafe fn iter_swap_7(mut a: adl_Iterator, mut b: adl_Iterator) {
    unimplemented!();
    if !(a.iter_swaps_).is_null() {
        (*a.iter_swaps_).prefix_inc();
    }
}
pub unsafe fn iter_move_8(mut iter: adl_Iterator) -> *mut () {
    if !(iter.iter_moves_).is_null() {
        (*iter.iter_moves_).prefix_inc();
    }
    return &mut (*(unsafe { adl_Iterator::operator_deref(&iter) }));
}
pub unsafe fn operator_eq_9(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return (((*lhs).ptr_) == ((*rhs).ptr_));
}
pub unsafe fn operator_cmp_10(lhs: *const adl_Iterator, rhs: *const adl_Iterator) -> () {
    return std::cmp::Ord::cmp(&((*lhs).ptr_), &((*rhs).ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct adl_Iterator {
    ptr_: *mut (),
    iter_moves_: *mut (),
    iter_swaps_: *mut (),
}
impl adl_Iterator {
    unsafe fn new_1(mut p: *mut (), mut iter_moves: *mut (), mut iter_swaps: *mut ()) -> Self {
        let mut this = Self {
            ptr_: p,
            iter_moves_: iter_moves,
            iter_swaps_: iter_swaps,
        };
        this
    }
    pub unsafe fn TrackMoves(mut p: *mut (), iter_moves: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { (iter_moves) }, { std::ptr::null_mut() });
    }
    pub unsafe fn TrackSwaps_4(iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ std::ptr::null_mut() }, { std::ptr::null_mut() }, {
            (iter_swaps)
        });
    }
    pub unsafe fn TrackSwaps_5(mut p: *mut (), iter_swaps: *mut ()) -> adl_Iterator {
        return adl_Iterator::new_1({ p }, { std::ptr::null_mut() }, { (iter_swaps) });
    }
    pub unsafe fn iter_moves(&self) -> () {
        unimplemented!();
        return (*self.iter_moves_);
    }
    pub unsafe fn iter_swaps(&self) -> () {
        unimplemented!();
        return (*self.iter_swaps_);
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn operator_sub_10(&self, mut n: ()) -> adl_Iterator {
        return adl_Iterator::new_1(
            { self.ptr_.offset(-((n) as isize)) },
            { self.iter_moves_ },
            { self.iter_swaps_ },
        );
    }
    pub unsafe fn operator_sub_11(&self, mut rhs: adl_Iterator) -> () {
        return (((self.ptr_ as usize - rhs.ptr_ as usize) / ::std::mem::size_of::<()>()) as ()
            as ());
    }
    pub unsafe fn operator_add_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_sub_assign(&mut self, mut n: ()) -> *mut adl_Iterator {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_inc_14(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_inc_15(&mut self, mut _a0: ()) -> adl_Iterator {
        let mut prev: adl_Iterator = (*(self as *mut adl_Iterator));
        self.ptr_.prefix_inc();
        return prev;
    }
    pub unsafe fn operator_dec_16(&mut self) -> *mut adl_Iterator {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut adl_Iterator));
    }
    pub unsafe fn operator_post_dec_17(&mut self, mut _a0: ()) -> adl_Iterator {
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
    it_: *mut *mut (),
}
impl rvalue_iterator_int_ptr_ {
    pub unsafe fn operator_cmp(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
        {
            let mut cmp: () = std::cmp::Ord::cmp(
                &((*(self as *const rvalue_iterator_int_ptr_)).it_),
                &((*_a0).it_),
            );
            if !(unimplemented!()) {
                return unimplemented!();
            }
        }
        return unimplemented!();
    }
    pub unsafe fn operator_eq(&self, _a0: *const rvalue_iterator_int_ptr_) -> () {
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
    pub unsafe fn new(mut what: *const ()) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_allocator_statistics {
    pub time_to_throw: (),
    pub throw_after: (),
    pub count: (),
    pub alloc_count: (),
    pub allocated_size: (),
    pub construct_count: (),
    pub destroy_count: (),
    pub copied: (),
    pub moved: (),
    pub converted: (),
}
impl test_allocator_statistics {
    pub unsafe fn clear(&mut self) {
        unimplemented!();
        self.count = 0_();
        self.time_to_throw = 0_();
        self.alloc_count = 0_();
        self.allocated_size = 0_();
        self.construct_count = 0_();
        self.destroy_count = 0_();
        self.throw_after = 2147483647_();
        (unsafe { test_allocator_statistics::clear_ctor_counters(self) });
    }
    pub unsafe fn clear_ctor_counters(&mut self) {
        self.copied = 0_();
        self.moved = 0_();
        self.converted = 0_();
    }
}
impl Default for test_allocator_statistics {
    fn default() -> Self {
        unsafe {
            test_allocator_statistics {
                time_to_throw: 0_(),
                throw_after: 2147483647_(),
                count: 0_(),
                alloc_count: 0_(),
                allocated_size: 0_(),
                construct_count: 0_(),
                destroy_count: 0_(),
                copied: 0_(),
                moved: 0_(),
                converted: 0_(),
            }
        }
    }
}
pub static mut destructed_value_11: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { -1_() });
pub static mut moved_value_12: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 2147483647_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_base {}
pub unsafe fn operator_eq_13(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return (((*x).data_) == ((*y).data_));
}
pub unsafe fn operator_ne_14(x: *const test_allocator_void_, y: *const test_allocator_void_) -> () {
    return !(unsafe {
        let _x: *const test_allocator_void_ = x;
        let _y: *const test_allocator_void_ = y;
        operator_eq_13(_x, _y)
    });
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct test_allocator_void_ {
    data_: (),
    id_: (),
    stats_: *mut test_allocator_statistics,
}
impl test_allocator_void_ {
    pub unsafe fn new_1(mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: 0_(),
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_2(mut data: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_3(mut data: (), mut stats: *mut test_allocator_statistics) -> Self {
        let mut this = Self {
            data_: data,
            id_: 0_(),
            stats_: stats,
        };
        this
    }
    pub unsafe fn new_4(mut data: (), mut id: ()) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn new_5(
        mut data: (),
        mut id: (),
        mut stats: *mut test_allocator_statistics,
    ) -> Self {
        let mut this = Self {
            data_: data,
            id_: id,
            stats_: stats,
        };
        this
    }
    pub unsafe fn copy_from(a: *const test_allocator_void_) -> Self {
        let mut this = Self {
            data_: (*a).data_,
            id_: (*a).id_,
            stats_: (*a).stats_,
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.data_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11));
        self.id_ = (*std::cell::LazyCell::force_mut(&mut *&raw mut destructed_value_11));
    }
    pub unsafe fn get_id(&self) -> () {
        return self.id_;
    }
    pub unsafe fn get_data(&self) -> () {
        return self.data_;
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const test_allocator_void_,
    ) -> *mut test_allocator_void_ {
        self.data_ = (*_a0).data_;
        self.id_ = (*_a0).id_;
        self.stats_ = (*_a0).stats_;
        return &mut (*(self as *mut test_allocator_void_));
    }
}
impl std::cmp::PartialEq for test_allocator_void_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_13(
                self as *const test_allocator_void_,
                other as *const test_allocator_void_,
            )
        }
    }
}
impl std::cmp::Eq for test_allocator_void_ {}
impl Clone for test_allocator_void_ {
    fn clone(&self) -> Self {
        unsafe { test_allocator_void_::copy_from(self as *const test_allocator_void_) }
    }
}
impl Default for test_allocator_void_ {
    fn default() -> Self {
        unsafe {
            test_allocator_void_ {
                data_: 0_(),
                id_: 0_(),
                stats_: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Ctor_Tag {}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, MoveAssignUnsafe, Default)]
pub struct Tag_X {}
impl Tag_X {
    pub unsafe fn new_1(mut _a0: Ctor_Tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_2(mut _a0: Ctor_Tag, _a1: *const Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn new_3(mut _a0: Ctor_Tag, _a1: *mut Tag_X) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Tag_X) -> *mut Tag_X {
        return &mut (*(self as *mut Tag_X));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct limited_alloc_handle_6_ {
    pub outstanding_: (),
    pub last_alloc_: (),
}
impl Default for limited_alloc_handle_6_ {
    fn default() -> Self {
        unsafe {
            limited_alloc_handle_6_ {
                outstanding_: 0_(),
                last_alloc_: unimplemented!(),
            }
        }
    }
}
pub unsafe fn make_thread_unsafe_shared_15()
-> detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ {
    let mut ptr: detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ =
        <detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__>::default();
    let _dtor_ptr = ScopedDestructorUnsafe::new(
        &raw mut ptr,
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__::destructor,
    );
    let mut alloc: () = unimplemented!();
    ptr.block = unimplemented!();
    unimplemented!();
    return detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__::copy_from({ &ptr });
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_6___control_block {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ref_count: 1_(),
            content: <limited_alloc_handle_6_>::default(),
        };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_6___control_block {
    pub ref_count: (),
    pub content: limited_alloc_handle_6_,
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_6___control_block {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_6___control_block {
                ref_count: 1_(),
                content: <limited_alloc_handle_6_>::default(),
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ {
    block: *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_6___control_block,
}
impl detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ {
    pub unsafe fn copy_from(
        other: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__,
    ) -> Self {
        let mut this = Self {
            block: (*other).block,
        };
        (*this.block).ref_count.prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.block).ref_count.prefix_dec();
        if (((*self.block).ref_count) != (0_())) {
            return;
        };
        let mut alloc: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    pub unsafe fn operator_arrow_7(&mut self) -> *mut limited_alloc_handle_6_ {
        return (&mut (*self.block).content as *mut limited_alloc_handle_6_);
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__,
    ) -> *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ {
        self.block = (*_a0).block;
        return &mut (*(self as *mut detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__));
    }
}
impl Clone for detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ {
    fn clone(&self) -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__::copy_from(
                self as *const detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__,
            )
        }
    }
}
impl Default for detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ {
    fn default() -> Self {
        unsafe {
            detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__ {
                block: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct limited_allocator_int__6__rebind_int_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct limited_allocator_int__6_ {
    handle_: detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__,
}
impl limited_allocator_int__6_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            handle_: (unsafe { make_thread_unsafe_shared_15() }),
        };
        this
    }
}
impl limited_allocator_int__6_ {
    pub unsafe fn destructor(&mut self) {
        detail_thread_unsafe_shared_ptr_limited_alloc_handle_6__::destructor(&mut self.handle_);
    }
}
impl Default for limited_allocator_int__6_ {
    fn default() -> Self {
        unsafe { limited_allocator_int__6_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_17))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_18))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_19)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_16)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_17)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_18)) = 0_();
    }
}
pub static mut outstanding_bytes_16: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_17: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_18: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_19: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct EmplaceConstructible_int_ {
    pub value: (),
}
impl EmplaceConstructible_int_ {
    pub unsafe fn new(mut xvalue: ()) -> Self {
        let mut this = Self { value: xvalue };
        this
    }
}
pub unsafe fn operator_eq_20(LHS: *const detail_TypeID, RHS: *const detail_TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_21(LHS: *const detail_TypeID, RHS: *const detail_TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_TypeID {
    m_id: *const (),
}
impl detail_TypeID {
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for detail_TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_20(self as *const detail_TypeID, other as *const detail_TypeID) }
    }
}
impl std::cmp::Eq for detail_TypeID {}
static mut dummy_addr_22: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 42_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct detail_TypeInfo_detail_ArgumentListID_int_ref__ {}
impl detail_TypeInfo_detail_ArgumentListID_int_ref__ {
    pub unsafe fn GetID() -> *const detail_TypeID {
        return &(*std::cell::LazyCell::force_mut(&mut *&raw mut id_23));
    }
}
pub unsafe fn makeTypeID_24() -> *const detail_TypeID {
    return (unsafe { detail_TypeInfo_detail_ArgumentListID_int_ref__::GetID() });
}
pub unsafe fn makeArgumentID_25() -> *const detail_TypeID {
    return (unsafe { makeTypeID_24() });
}
static mut id_23: std::cell::LazyCell<detail_TypeID> = std::cell::LazyCell::new(|| unsafe {
    detail_TypeID::new({
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_addr_22)) as *const ())
    })
});
#[repr(C)]
#[derive(FnPtrArg)]
pub struct AllocatorConstructController {
    pub m_expected_args: *const detail_TypeID,
    pub m_allow_constructions: (),
    pub m_allow_unchecked: (),
    pub m_expected_count: (),
}
impl AllocatorConstructController {
    pub unsafe fn clear(&mut self) {
        self.m_expected_args = std::ptr::null();
        self.m_expected_count = -1_();
    }
    pub unsafe fn check(&mut self, tid: *const detail_TypeID) -> () {
        if !(!(self.m_expected_args).is_null()) {
            unimplemented!();
            return self.m_allow_unchecked;
        }
        let mut res: () = (unsafe {
            let _LHS: *const detail_TypeID = &(*self.m_expected_args);
            let _RHS: *const detail_TypeID = tid;
            operator_eq_20(_LHS, _RHS)
        });
        if ((self.m_expected_count) == (-1_())) || ((self.m_expected_count.prefix_dec()) == (-1_()))
        {
            self.m_expected_args = std::ptr::null();
        }
        return res;
    }
    pub unsafe fn unchecked(&mut self) -> () {
        return !((self.m_expected_args).is_null());
    }
    pub unsafe fn expect(&mut self, tid: *const detail_TypeID) {
        unimplemented!();
        self.m_expected_args = (tid);
    }
    pub unsafe fn isInAllocatorConstruct(&self) -> () {
        return self.m_allow_constructions;
    }
    pub unsafe fn inAllocatorConstruct(&mut self, mut value: Option<()>) {
        let mut value: () = value.unwrap_or_else(|| unsafe { true });
        self.m_allow_constructions = value;
    }
    pub unsafe fn allowUnchecked(&mut self, mut value: Option<()>) {
        let mut value: () = value.unwrap_or_else(|| unsafe { true });
        self.m_allow_unchecked = value;
    }
    pub unsafe fn reset(&mut self) {
        self.m_allow_constructions = false;
        self.m_expected_args = std::ptr::null();
        self.m_allow_unchecked = false;
        self.m_expected_count = -1_();
    }
    unsafe fn new() -> Self {
        let mut this = Self {
            m_expected_args: std::ptr::null(),
            m_allow_constructions: unimplemented!(),
            m_allow_unchecked: unimplemented!(),
            m_expected_count: unimplemented!(),
        };
        (unsafe { AllocatorConstructController::reset(&mut this) });
        this
    }
    pub unsafe fn expect_5_int_ref(&mut self, mut times: ()) {
        unimplemented!();
        unimplemented!();
        self.m_expected_count = ((times) - (1_()));
        self.m_expected_args = (unsafe { makeArgumentID_25() });
    }
    pub unsafe fn check_6_int_ref(&mut self) -> () {
        return (unsafe {
            AllocatorConstructController::check(self, (unsafe { makeArgumentID_25() }))
        });
    }
}
impl Default for AllocatorConstructController {
    fn default() -> Self {
        unsafe { AllocatorConstructController::new() }
    }
}
pub unsafe fn getConstructController_26() -> *mut AllocatorConstructController {
    return (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut c_27))
        as *mut AllocatorConstructController);
}
static mut c_27: std::cell::LazyCell<AllocatorConstructController> =
    std::cell::LazyCell::new(|| unsafe { AllocatorConstructController::new() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct ExpectConstructGuard_int_ref_ {}
impl ExpectConstructGuard_int_ref_ {
    pub unsafe fn new(mut N: ()) -> Self {
        let mut this = Self {};
        let mut CC: *mut AllocatorConstructController = (unsafe { getConstructController_26() });
        unimplemented!();
        (unsafe { AllocatorConstructController::expect_5_int_ref(&mut (*CC), N) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerTestAllocator_int__int_ {
    pub construct_called: (),
    pub destroy_called: (),
    pub controller: *mut AllocatorConstructController,
}
impl ContainerTestAllocator_int__int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerTestAllocator_float__int_ {
    pub construct_called: (),
    pub destroy_called: (),
    pub controller: *mut AllocatorConstructController,
}
impl ContainerTestAllocator_float__int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__ {
    pub construct_called: (),
    pub destroy_called: (),
    pub controller: *mut AllocatorConstructController,
}
impl ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__ {
    pub unsafe fn new_1() -> Self {
        let mut this = Self {
            construct_called: unimplemented!(),
            destroy_called: unimplemented!(),
            controller: (unsafe { getConstructController_26() }),
        };
        this
    }
}
impl Default for ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__ {
    fn default() -> Self {
        unsafe { ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__::new_1() }
    }
}
const _: () = assert!(true, "std::is_same<A1T::rebind_traits<float>, A2T>::value");
const _: () = assert!(true, "std::is_same<A2T::rebind_traits<int>, A1T>::value");
#[repr(C)]
#[derive(FnPtrArg, DestructorUnsafe, Default)]
pub struct CopyInsertable_1_ {
    pub data: (),
    pub copied_once: (),
    pub constructed_under_allocator: (),
}
impl CopyInsertable_1_ {
    pub unsafe fn new_1(mut val: ()) -> Self {
        let mut this = Self {
            data: val,
            copied_once: false,
            constructed_under_allocator: false,
        };
        if (unsafe {
            AllocatorConstructController::isInAllocatorConstruct(
                &(*(unsafe { getConstructController_26() }).cast_const()),
            )
        }) {
            this.copied_once = true;
            this.constructed_under_allocator = true;
        }
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
    }
}
pub unsafe fn basic_test_28() {
    {
        let mut a: [(); 4] = [0_(), 1_(), 2_(), 3_()];
        let mut l: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        let mut i: () = unimplemented!();
        let mut e: () = unimplemented!();
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            {
                unimplemented!();
                j.prefix_inc()
            };
        }
    }
    {
        let mut a: [(); 4] = [0_(), 1_(), 2_(), 3_()];
        let mut l: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        let mut i: () = unimplemented!();
        let mut e: () = unimplemented!();
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            {
                unimplemented!();
                j.prefix_inc()
            };
        }
    }
    {
        let mut a: [(); 4] = [0_(), 1_(), 2_(), 3_()];
        let mut l: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        let mut i: () = unimplemented!();
        let mut e: () = unimplemented!();
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            {
                unimplemented!();
                j.prefix_inc()
            };
        }
    }
    {
        let mut a: [(); 4] = [0_(), 1_(), 2_(), 3_()];
        let mut l: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        let mut i: () = unimplemented!();
        let mut e: () = unimplemented!();
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            {
                unimplemented!();
                j.prefix_inc()
            };
        }
    }
    {
        let mut a: [(); 4] = [0_(), 1_(), 2_(), 3_()];
        let mut l: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut j: () = 0_();
        let mut i: () = unimplemented!();
        let mut e: () = unimplemented!();
        'loop_: while !(unimplemented!()) {
            unimplemented!();
            {
                unimplemented!();
                j.prefix_inc()
            };
        }
    }
}
pub unsafe fn test_emplacable_concept_29() {
    let mut arr1: [(); 1] = [42_()];
    let mut arr2: [(); 3] = [1_(), 101_(), 42_()];
    {
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_emplacable_concept_with_alloc_30() {
    let mut arr1: [(); 1] = [42_()];
    let mut arr2: [(); 3] = [1_(), 101_(), 42_()];
    {
        let mut a: () = unimplemented!();
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
        }
        {
            let mut v: () = unimplemented!();
            let mut I: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_ctor_under_alloc_31() {
    let mut arr1: [(); 1] = [42_()];
    let mut arr2: [(); 3] = [1_(), 101_(), 42_()];
    {
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 1_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 3_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
    }
    {
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 1_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 3_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
    }
}
pub unsafe fn test_ctor_under_alloc_with_alloc_32() {
    let mut arr1: [(); 1] = [42_()];
    let mut arr2: [(); 3] = [1_(), 101_(), 42_()];
    {
        let mut a: ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__ =
            ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__::new_1();
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 1_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 3_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
    }
    {
        let mut a: ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__ =
            ContainerTestAllocator_CopyInsertable_1___CopyInsertable_1__::new_1();
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 1_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
        {
            let mut G: ExpectConstructGuard_int_ref_ = ExpectConstructGuard_int_ref_::new({ 3_() });
            let _dtor_G =
                ScopedDestructorUnsafe::new(&raw mut G, ExpectConstructGuard_int_ref_::destructor);
            let mut v: () = unimplemented!();
        }
    }
}
pub unsafe fn test_33() -> () {
    (unsafe { basic_test_28() });
    (unsafe { test_emplacable_concept_29() });
    (unsafe { test_emplacable_concept_with_alloc_30() });
    {
        (unsafe { test_ctor_under_alloc_31() });
        (unsafe { test_ctor_under_alloc_with_alloc_32() });
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const destructed_value_11);
    std::cell::LazyCell::force(&*&raw const moved_value_12);
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_16);
    std::cell::LazyCell::force(&*&raw const alloc_count_17);
    std::cell::LazyCell::force(&*&raw const dealloc_count_18);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_19);
    std::cell::LazyCell::force(&*&raw const dummy_addr_22);
}
